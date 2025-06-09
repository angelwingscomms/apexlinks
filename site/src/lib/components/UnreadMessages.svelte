<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { fade, fly } from 'svelte/transition';
  import { currentUser } from '$lib/stores/userStore';
  import { goto } from '$app/navigation';
  
  // API URL configuration - assuming localhost:8000 for now
  const API_URL = 'http://localhost:8000';
  
  let unreadMessages = $state<any[]>([]);
  let isLoading = $state(true);
  let error = $state<string | null>(null);
  let pollingInterval: number | null = null;
  
  onMount(() => {
    loadUnreadMessages();
    
    // Poll for new messages every 30 seconds
    pollingInterval = window.setInterval(() => {
      loadUnreadMessages();
    }, 30000);
  });
  
  onDestroy(() => {
    if (pollingInterval !== null) {
      clearInterval(pollingInterval);
    }
  });
  
  async function loadUnreadMessages() {
    if (!$currentUser) return;
    
    try {
      const response = await fetch(`${API_URL}/chat/unread`, {
        headers: {
          'Authorization': `Bearer ${$currentUser.id}`
        }
      });
      
      if (!response.ok) {
        throw new Error(`Failed to fetch unread messages: ${response.statusText}`);
      }
      
      const data = await response.json();
      unreadMessages = data;
      
      // Show browser notification if there are new messages and the page is not focused
      if (unreadMessages.length > 0 && document.hidden) {
        showBrowserNotification();
      }
    } catch (err) {
      console.error('Error fetching unread messages:', err);
      error = err instanceof Error ? err.message : 'Failed to load unread messages';
    } finally {
      isLoading = false;
    }
  }
  
  function formatTimestamp(timestamp: number): string {
    const date = new Date(timestamp * 1000);
    return date.toLocaleTimeString();
  }
  
  async function markAsRead(messageId: string) {
    if (!$currentUser) return;
    
    try {
      await fetch(`${API_URL}/chat/mark-read`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${$currentUser.id}`
        },
        body: JSON.stringify({
          message_ids: [messageId]
        })
      });
      
      // Remove from local list
      unreadMessages = unreadMessages.filter(msg => msg.id !== messageId);
    } catch (err) {
      console.error('Error marking message as read:', err);
    }
  }
  
  function navigateToChat(sessionId: string, partnerId: string) {
    // Mark all messages from this session as read
    const sessionMessages = unreadMessages.filter(msg => msg.session_id === sessionId);
    if (sessionMessages.length > 0) {
      const messageIds = sessionMessages.map(msg => msg.id);
      
      fetch(`${API_URL}/chat/mark-read`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${$currentUser.id}`
        },
        body: JSON.stringify({
          message_ids: messageIds
        })
      }).catch(err => console.error('Error marking messages as read:', err));
      
      // Remove from local list
      unreadMessages = unreadMessages.filter(msg => msg.session_id !== sessionId);
    }
    
    // Navigate to chat
    goto(`/chat?session=${sessionId}&partner=${partnerId}`);
  }
  
  function showBrowserNotification() {
    if (!$currentUser) return;
    
    // Check if browser supports notifications and permission is granted
    if ('Notification' in window) {
      if (Notification.permission === 'granted') {
        new Notification('New Messages', {
          body: `You have ${unreadMessages.length} unread message(s)`,
          icon: '/logo.png'
        });
      } else if (Notification.permission !== 'denied') {
        Notification.requestPermission().then(permission => {
          if (permission === 'granted') {
            new Notification('New Messages', {
              body: `You have ${unreadMessages.length} unread message(s)`,
              icon: '/logo.png'
            });
          }
        });
      }
    }
  }
</script>

<div class="glass-sm p-4 rounded-xl">
  <h3 class="text-xl font-semibold mb-4 dreamy-text">
    Unread Messages
    {#if unreadMessages.length > 0}
      <span class="bg-red-500 text-white text-xs rounded-full px-2 py-1 ml-2">
        {unreadMessages.length}
      </span>
    {/if}
  </h3>
  
  {#if isLoading}
    <div class="flex justify-center py-8">
      <div class="animate-spin h-8 w-8 border-4 border-red-500 rounded-full border-t-transparent"></div>
    </div>
  {:else if unreadMessages.length === 0}
    <div class="text-center py-6 text-gray-600">
      No unread messages
    </div>
  {:else}
    <div class="space-y-3 max-h-80 overflow-y-auto">
      {#each unreadMessages as message}
        <div 
          class="glass-sm p-3 rounded-lg relative group"
          transition:fly={{ y: 10, duration: 200 }}
        >
          <button 
            class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity text-gray-500 hover:text-red-500"
            on:click={() => markAsRead(message.id)}
            aria-label="Mark as read"
          >
            ✓
          </button>
          
          <div class="mb-2 line-clamp-2">{message.message}</div>
          
          <div class="flex justify-between text-xs text-gray-500">
            <span>From: {message.sender_id}</span>
            <span>{formatTimestamp(message.timestamp)}</span>
          </div>
          
          <button 
            class="mt-2 w-full btn-sm bg-red-100 hover:bg-red-200 text-red-800 text-sm"
            on:click={() => navigateToChat(message.session_id, message.sender_id)}
          >
            View Conversation
          </button>
        </div>
      {/each}
    </div>
  {/if}
</div> 