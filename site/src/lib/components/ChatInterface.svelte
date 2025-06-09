<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { currentUser } from '$lib/stores/userStore';
  import { fade, fly } from 'svelte/transition';
  import { createEventDispatcher } from 'svelte';
  
  const dispatch = createEventDispatcher<{
    sessionCreated: { sessionId: string, partnerId: string }
  }>();
  
  const { partnerId, sessionId = null } = $props<{
    partnerId: string;
    sessionId: string | null;
  }>();
  
  let websocket: WebSocket | null = null;
  let messages = $state<any[]>([]);
  let messageInput = $state('');
  let isConnected = $state(false);
  let connectionError = $state<string | null>(null);
  let showNotification = $state(false);
  let notificationMessage = $state('');
  let notificationTimeout: any = null;
  
  // API URL configuration
  const API_URL = 'http://localhost:8000';
  
  // Connect to WebSocket when the component mounts
  onMount(async () => {
    connectWebSocket();
    
    // If we have a session ID, fetch the message history
    if (sessionId) {
      await fetchMessageHistory();
    }
  });
  
  onDestroy(() => {
    disconnectWebSocket();
  });
  
  function connectWebSocket() {
    // Close any existing connection
    if (websocket) {
      websocket.close();
    }
    
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${protocol}//${window.location.hostname}:8000/chat/ws`;
    
    websocket = new WebSocket(wsUrl);
    
    websocket.onopen = () => {
      isConnected = true;
      connectionError = null;
      
      // If we have a session ID, join that session
      if (sessionId) {
        sendControlMessage('join_session', {
          session_id: sessionId,
          user_id: $currentUser?.id
        });
      } else {
        // Otherwise, initiate a match
        initiateMatch();
      }
    };
    
    websocket.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        
        // If this is a match response
        if (data.match_found !== undefined) {
          handleMatchResponse(data);
          return;
        }
        
        // Regular chat message
        messages = [...messages, data];
        
        // Show notification if not the current tab
        if (document.hidden && data.sender_id !== $currentUser?.id && data.message_type === 'Text') {
          showNotificationMessage(`New message from ${data.sender_id}`);
        }
      } catch (error) {
        console.error('Error parsing message:', error);
      }
    };
    
    websocket.onclose = () => {
      isConnected = false;
      
      // Attempt to reconnect after a delay
      setTimeout(() => {
        if (!isConnected) {
          connectWebSocket();
        }
      }, 3000);
    };
    
    websocket.onerror = (error) => {
      connectionError = 'Connection error. Trying to reconnect...';
      console.error('WebSocket error:', error);
    };
  }
  
  function disconnectWebSocket() {
    if (sessionId && websocket && isConnected) {
      sendControlMessage('leave_session', {
        session_id: sessionId,
        user_id: $currentUser?.id
      });
    }
    
    if (websocket) {
      websocket.close();
      websocket = null;
    }
    
    if (notificationTimeout) {
      clearTimeout(notificationTimeout);
    }
  }
  
  async function initiateMatch() {
    if (!$currentUser) return;
    
    // Send match request with user profile data
    sendControlMessage('find_match', {
      user_id: $currentUser.id,
      description: $currentUser.description || '',
      interests: [], // User interests are not defined in the User type
      age_range: $currentUser.age ? `${$currentUser.age}` : undefined
    });
  }
  
  function handleMatchResponse(data: any) {
    const updatedSessionId = data.session_id;
    const updatedPartnerId = data.partner_id;
    
    if (data.match_found && updatedSessionId) {
      // Dispatch event to parent component
      dispatch('sessionCreated', {
        sessionId: updatedSessionId,
        partnerId: updatedPartnerId
      });
      
      // Join the session
      sendControlMessage('join_session', {
        session_id: updatedSessionId,
        user_id: $currentUser?.id
      });
      
      showNotificationMessage('Match found! You can start chatting.');
    } else {
      showNotificationMessage(data.message || 'No match found. Try again later.');
    }
  }
  
  function sendMessage() {
    if (!messageInput.trim() || !isConnected || !sessionId || !$currentUser) return;
    
    const message = {
      type: 'chat',
      session_id: sessionId,
      user_id: $currentUser.id,
      message: messageInput.trim()
    };
    
    if (websocket) {
      websocket.send(JSON.stringify(message));
      messageInput = '';
    }
  }
  
  function sendControlMessage(type: string, data: any) {
    if (!isConnected || !websocket) return;
    
    const message = {
      type,
      ...data
    };
    
    websocket.send(JSON.stringify(message));
  }
  
  function showNotificationMessage(message: string) {
    notificationMessage = message;
    showNotification = true;
    
    if (notificationTimeout) {
      clearTimeout(notificationTimeout);
    }
    
    notificationTimeout = setTimeout(() => {
      showNotification = false;
    }, 5000);
  }
  
  function getMessageClass(message: any) {
    if (message.message_type === 'System' || 
        message.message_type === 'UserConnected' || 
        message.message_type === 'UserDisconnected') {
      return 'system-message';
    }
    
    return message.sender_id === $currentUser?.id ? 'sent-message' : 'received-message';
  }
  
  // Add this function to fetch message history
  async function fetchMessageHistory() {
    if (!sessionId || !$currentUser) return;
    
    try {
      const response = await fetch(`${API_URL}/chat/messages/${sessionId}`, {
        headers: {
          'Authorization': `Bearer ${$currentUser.id}`
        }
      });
      
      if (!response.ok) {
        throw new Error(`Failed to fetch messages: ${response.statusText}`);
      }
      
      const data = await response.json();
      messages = data;
    } catch (error) {
      console.error('Error fetching message history:', error);
      showNotificationMessage('Failed to load message history');
    }
  }
</script>

<div class="glass h-full flex flex-col">
  <!-- Chat header -->
  <div class="p-4 border-b border-red-200 flex justify-between items-center">
    <h2 class="text-xl font-semibold dreamy-text">
      {sessionId ? 'Chat Session' : 'Looking for a match...'}
    </h2>
    <div class="flex items-center">
      {#if isConnected}
        <span class="bg-green-500 h-3 w-3 rounded-full mr-2 animate-pulse"></span>
        <span class="text-sm text-green-600">Connected</span>
      {:else}
        <span class="bg-red-500 h-3 w-3 rounded-full mr-2"></span>
        <span class="text-sm text-red-500">Disconnected</span>
      {/if}
    </div>
  </div>
  
  <!-- Chat messages -->
  <div class="flex-1 p-4 overflow-y-auto">
    {#if messages.length === 0}
      <div class="flex items-center justify-center h-full">
        <div class="text-center">
          <div class="glass-sm h-16 w-16 rounded-full mx-auto mb-4 flex items-center justify-center">
            <span class="text-red-500 text-2xl animate-pulse">💬</span>
          </div>
          <p class="text-gray-600">
            {sessionId ? 'No messages yet. Start the conversation!' : 'Waiting to match with someone...'}
          </p>
        </div>
      </div>
    {:else}
      <div class="space-y-4">
        {#each messages as message}
          <div 
            class={`${getMessageClass(message)} animate-in fade-in duration-300`} 
            class:glass-sm={message.message_type === 'Text'}
          >
            {#if message.message_type === 'System' || message.message_type === 'UserConnected' || message.message_type === 'UserDisconnected'}
              <div class="text-center py-2 px-4 text-sm text-gray-500 italic">
                {message.message}
              </div>
            {:else}
              <div class="p-3">
                <p>{message.message}</p>
                <div class="text-xs text-gray-500 mt-1 text-right">
                  {new Date(message.timestamp * 1000).toLocaleTimeString()}
                </div>
              </div>
            {/if}
          </div>
        {/each}
      </div>
    {/if}
  </div>
  
  <!-- Message input -->
  <div class="p-4 border-t border-red-200">
    {#if sessionId}
      <form on:submit|preventDefault={sendMessage} class="flex gap-2">
        <input
          type="text"
          bind:value={messageInput}
          placeholder="Type your message..."
          class="input-glass flex-1"
          disabled={!isConnected}
        />
        <button 
          type="submit" 
          class="btn-primary"
          disabled={!isConnected || !messageInput.trim()}
        >
          Send
        </button>
      </form>
    {:else}
      <button 
        class="btn-primary w-full"
        on:click={initiateMatch}
        disabled={!isConnected}
      >
        Find a Match
      </button>
    {/if}
  </div>
</div>

<!-- Notification toast -->
{#if showNotification}
  <div 
    class="fixed bottom-4 right-4 glass-sm p-4 text-gray-800 max-w-sm z-50"
    transition:fly={{ y: 50, duration: 300 }}
  >
    <div class="flex items-center">
      <span class="mr-2 text-red-500">💬</span>
      <p>{notificationMessage}</p>
    </div>
    <button 
      class="absolute top-1 right-1 text-gray-500 hover:text-gray-800"
      on:click={() => showNotification = false}
    >
      &times;
    </button>
  </div>
{/if}

<style>
  .sent-message {
    margin-left: auto;
    max-width: 80%;
    background: rgba(239, 68, 68, 0.15);
    border-radius: 1rem 1rem 0 1rem;
  }
  
  .received-message {
    margin-right: auto;
    max-width: 80%;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 1rem 1rem 1rem 0;
  }
  
  .system-message {
    width: 100%;
    text-align: center;
  }
  
  .animate-in {
    animation: fadeIn 0.3s ease-out;
  }
  
  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style> 