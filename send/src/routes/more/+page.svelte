<script lang="ts">
  const faqItems = [
    {
      question: "How does SendMe work?",
      answer: "SendMe connects you with local service providers for your vehicle needs. Browse services, read reviews, and request assistance with just a few clicks."
    },
    {
      question: "Is SendMe available in my area?",
      answer: "We're currently available in major cities across the United States. Enter your location to see if service is available in your area."
    },
    {
      question: "How do I pay for services?",
      answer: "Payment is handled securely through our app after service is completed. We accept all major credit cards and digital payment methods."
    },
    {
      question: "What if I'm not satisfied with the service?",
      answer: "Your satisfaction is our priority. If you're unhappy with a service, please contact our support team and we'll work to make it right."
    },
    {
      question: "How do I become a service provider?",
      answer: "If you're a qualified service provider interested in joining our network, visit our Partners page to learn about the application process."
    }
  ];
  
  let expandedIndex = $state<number | null>(null);
  
  const toggleFaq = (index: number) => {
    expandedIndex = expandedIndex === index ? null : index;
  };
</script>

<div class="container mx-auto px-4 py-8">
  <h1 class="text-3xl font-bold mb-8">More Information</h1>
  
  <div class="neumorphic p-6 mb-8">
    <h2 class="text-2xl font-bold mb-6">About SendMe</h2>
    <p class="mb-4">
      SendMe is a revolutionary platform connecting vehicle owners with trusted service providers. 
      Our mission is to simplify vehicle maintenance and emergency assistance by bringing quality 
      services directly to you, whenever and wherever you need them.
    </p>
    <p class="mb-4">
      Founded in 2023, we've quickly grown to become the preferred vehicle service platform 
      for thousands of users nationwide. Our network includes certified mechanics, towing 
      services, car wash providers, and fuel delivery specialists.
    </p>
    <p>
      At SendMe, we believe that vehicle care should be convenient, transparent, and stress-free. 
      That's why we carefully vet all service providers and maintain high standards for quality 
      and reliability.
    </p>
  </div>
  
  <div class="neumorphic p-6 mb-8">
    <h2 class="text-2xl font-bold mb-6">Frequently Asked Questions</h2>
    
    <div class="space-y-4">
      {#each faqItems as faq, index}
        <div class="border-b border-gray-200 pb-4">
          <button 
            onclick={() => toggleFaq(index)}
            class="w-full text-left flex justify-between items-center py-2"
          >
            <span class="font-medium">{faq.question}</span>
            <span class="text-xl">
              {expandedIndex === index ? '−' : '+'}
            </span>
          </button>
          
          {#if expandedIndex === index}
            <div class="mt-2 text-secondary">
              {faq.answer}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  </div>
  
  <div class="neumorphic p-6">
    <h2 class="text-2xl font-bold mb-6">Our Partners</h2>
    
    <div class="grid grid-cols-2 md:grid-cols-4 gap-6">
      <div class="aspect-square bg-gradient-dark rounded-lg flex items-center justify-center">
        <span class="text-primary font-bold">AutoCare</span>
      </div>
      <div class="aspect-square bg-gradient-dark rounded-lg flex items-center justify-center">
        <span class="text-primary font-bold">SpeedTow</span>
      </div>
      <div class="aspect-square bg-gradient-dark rounded-lg flex items-center justify-center">
        <span class="text-primary font-bold">GlowWash</span>
      </div>
      <div class="aspect-square bg-gradient-dark rounded-lg flex items-center justify-center">
        <span class="text-primary font-bold">FuelDrop</span>
      </div>
    </div>
  </div>
</div> 