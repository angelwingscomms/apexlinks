import { animate } from 'animejs';

/**
 * Animation utilities for site-wide use
 * Using Anime.js for smooth, performant animations
 */

/**
 * Initialize site-wide animations
 * Call this function in the +layout.svelte file to set up global animations
 */
export function initializeAnimations() {
  // Add entry animations to all elements with the data-animate attribute
  const animatedElements = document.querySelectorAll('[data-animate]');
  animatedElements.forEach((el) => {
    const animationType = el.getAttribute('data-animate');
    if (animationType) {
      applyEntryAnimation(el, animationType);
    }
  });

  // Initialize any magnetic elements
  initializeMagneticElements();
  
  // Initialize any parallax elements
  initializeParallaxElements();
}

/**
 * Apply an entry animation to an element
 * @param {HTMLElement} element - The element to animate
 * @param {string} type - The type of animation to apply (fade, slide, scale, etc.)
 * @param {Object} options - Additional animation options
 */
export function applyEntryAnimation(element, type = 'fade', options = {}) {
  // Default options
  const defaultOptions = {
    duration: 800,
    easing: 'cubicBezier(0.25, 0.1, 0.25, 1.5)',
    delay: element.dataset.delay || 0,
  };
  
  // Merge options
  const animationOptions = { ...defaultOptions, ...options };
  
  // Set initial opacity to 0
  element.style.opacity = 0;
  
  // Animation properties based on type
  let animationProps = {};
  
  switch (type) {
    case 'fade':
      animationProps = {
        opacity: [0, 1],
        duration: animationOptions.duration,
      };
      break;
    case 'slide-up':
      animationProps = {
        opacity: [0, 1],
        translateY: [20, 0],
        duration: animationOptions.duration,
      };
      break;
    case 'slide-down':
      animationProps = {
        opacity: [0, 1],
        translateY: [-20, 0],
        duration: animationOptions.duration,
      };
      break;
    case 'slide-left':
      animationProps = {
        opacity: [0, 1],
        translateX: [20, 0],
        duration: animationOptions.duration,
      };
      break;
    case 'slide-right':
      animationProps = {
        opacity: [0, 1],
        translateX: [-20, 0],
        duration: animationOptions.duration,
      };
      break;
    case 'scale':
      animationProps = {
        opacity: [0, 1],
        scale: [0.9, 1],
        duration: animationOptions.duration,
      };
      break;
    case 'flip':
      animationProps = {
        opacity: [0, 1],
        rotateX: [90, 0],
        duration: animationOptions.duration,
      };
      break;
    default:
      animationProps = {
        opacity: [0, 1],
        duration: animationOptions.duration,
      };
  }
  
  // Create the animation when element is in viewport
  const observer = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        // Import anime dynamically to avoid SSR issues
        import('animejs').then(({ default: anime }) => {
          anime({
            targets: element,
            ...animationProps,
            easing: animationOptions.easing,
            delay: animationOptions.delay,
          });
        });
        
        // Unobserve after animation starts
        observer.unobserve(element);
      }
    });
  }, { threshold: 0.1 });
  
  observer.observe(element);
}

/**
 * Stagger animation for a group of elements
 * @param {NodeList|Array} elements - The elements to animate
 * @param {Object} options - Animation options
 */
export function staggerAnimation(elements, options = {}) {
  const defaultOptions = {
    opacity: [0, 1],
    translateY: [10, 0],
    delay: anime.stagger(50),
    duration: 800,
    easing: 'cubicBezier(0.25, 0.1, 0.25, 1.5)',
  };
  
  const animationOptions = { ...defaultOptions, ...options };
  
  // Create staggered animation when elements are in viewport
  const observer = new IntersectionObserver((entries) => {
    if (entries[0].isIntersecting) {
      // Import anime dynamically to avoid SSR issues
      import('animejs').then(({ default: anime }) => {
        anime({
          targets: elements,
          ...animationOptions,
        });
      });
      
      // Unobserve after animation starts
      observer.unobserve(entries[0].target);
    }
  }, { threshold: 0.1 });
  
  if (elements.length > 0) {
    observer.observe(elements[0]);
  }
}

/**
 * Create a hover animation for an element
 * @param {HTMLElement} element - The element to animate
 * @param {Object} options - Animation options
 */
export function createHoverAnimation(element, options = {}) {
  const defaultOptions = {
    scale: 1.05,
    duration: 400,
    easing: 'cubicBezier(0.25, 0.1, 0.25, 1.5)',
  };
  
  const animationOptions = { ...defaultOptions, ...options };
  
  // Import anime dynamically to avoid SSR issues
  import('animejs').then(({ default: anime }) => {
    // Create hover in animation
    element.addEventListener('mouseenter', () => {
      anime({
        targets: element,
        scale: animationOptions.scale,
        duration: animationOptions.duration,
        easing: animationOptions.easing,
      });
    });
    
    // Create hover out animation
    element.addEventListener('mouseleave', () => {
      anime({
        targets: element,
        scale: 1,
        duration: animationOptions.duration,
        easing: animationOptions.easing,
      });
    });
  });
}

/**
 * Initialize magnetic elements
 */
function initializeMagneticElements() {
  const magneticElements = document.querySelectorAll('.magnetic-element');
  
  magneticElements.forEach((element) => {
    // Get the strength of the magnetic effect (or use default)
    const strength = element.dataset.magneticStrength || 0.3;
    
    element.addEventListener('mousemove', (e) => {
      // Import anime dynamically to avoid SSR issues
      import('animejs').then(({ default: anime }) => {
        const rect = element.getBoundingClientRect();
        const x = e.clientX - rect.left - rect.width / 2;
        const y = e.clientY - rect.top - rect.height / 2;
        
        anime({
          targets: element,
          translateX: x * strength,
          translateY: y * strength,
          duration: 400,
          easing: 'easeOutElastic(1, .8)',
        });
      });
    });
    
    element.addEventListener('mouseleave', () => {
      // Import anime dynamically to avoid SSR issues
      import('animejs').then(({ default: anime }) => {
        anime({
          targets: element,
          translateX: 0,
          translateY: 0,
          duration: 800,
          easing: 'easeOutElastic(1, .8)',
        });
      });
    });
  });
}

/**
 * Initialize parallax elements
 */
function initializeParallaxElements() {
  const parallaxElements = document.querySelectorAll('[data-parallax]');
  
  window.addEventListener('scroll', () => {
    const scrollY = window.scrollY;
    
    parallaxElements.forEach((element) => {
      const speed = element.dataset.parallax || 0.2;
      const offsetY = scrollY * speed;
      
      // Apply parallax effect
      element.style.transform = `translateY(${offsetY}px)`;
    });
  });
}

/**
 * Create a text animation that reveals one character at a time
 * @param {HTMLElement} element - The element containing the text
 * @param {Object} options - Animation options
 */
export function createTextAnimation(element, options = {}) {
  const defaultOptions = {
    duration: 1200,
    delay: anime.stagger(30),
    easing: 'cubicBezier(0.25, 0.1, 0.25, 1.5)',
  };
  
  const animationOptions = { ...defaultOptions, ...options };
  
  // Get the text content
  const text = element.textContent;
  
  // Split the text into individual characters
  element.innerHTML = text.split('').map(char => {
    return char === ' ' 
      ? '<span class="whitespace">&nbsp;</span>' 
      : `<span class="char">${char}</span>`;
  }).join('');
  
  // Get all character elements
  const characters = element.querySelectorAll('.char');
  
  // Create the animation when element is in viewport
  const observer = new IntersectionObserver((entries) => {
    if (entries[0].isIntersecting) {
      // Set initial opacity to 0 for all characters
      characters.forEach(char => {
        char.style.opacity = 0;
        char.style.display = 'inline-block';
      });
      
      // Import anime dynamically to avoid SSR issues
      import('animejs').then(({ default: anime }) => {
        anime({
          targets: characters,
          opacity: [0, 1],
          translateY: [20, 0],
          translateZ: 0,
          duration: animationOptions.duration,
          delay: animationOptions.delay,
          easing: animationOptions.easing,
        });
      });
      
      // Unobserve after animation starts
      observer.unobserve(element);
    }
  }, { threshold: 0.1 });
  
  observer.observe(element);
}

/**
 * Create a path drawing animation for SVG elements
 * @param {HTMLElement} svgElement - The SVG element to animate
 * @param {Object} options - Animation options
 */
export function createPathAnimation(svgElement, options = {}) {
  const defaultOptions = {
    duration: 2000,
    easing: 'cubicBezier(0.5, 0, 0.5, 1)',
    delay: anime.stagger(100),
  };
  
  const animationOptions = { ...defaultOptions, ...options };
  
  // Get all path elements
  const paths = svgElement.querySelectorAll('path');
  
  // Set initial state
  paths.forEach(path => {
    const length = path.getTotalLength();
    path.style.strokeDasharray = length;
    path.style.strokeDashoffset = length;
  });
  
  // Create the animation when element is in viewport
  const observer = new IntersectionObserver((entries) => {
    if (entries[0].isIntersecting) {
      // Import anime dynamically to avoid SSR issues
      import('animejs').then(({ default: anime }) => {
        anime({
          targets: paths,
          strokeDashoffset: [anime.setDashoffset, 0],
          duration: animationOptions.duration,
          easing: animationOptions.easing,
          delay: animationOptions.delay,
        });
      });
      
      // Unobserve after animation starts
      observer.unobserve(svgElement);
    }
  }, { threshold: 0.1 });
  
  observer.observe(svgElement);
}

/**
 * Create a scroll-triggered timeline animation
 * @param {Array} animations - Array of animation objects with targets and properties
 * @param {Object} options - Global options for all animations
 */
export function createScrollTimeline(animations, options = {}) {
  const defaultOptions = {
    easing: 'cubicBezier(0.5, 0, 0.5, 1)',
    duration: 800,
  };
  
  const timelineOptions = { ...defaultOptions, ...options };
  
  animations.forEach(animation => {
    const { targets, properties } = animation;
    const elements = document.querySelectorAll(targets);
    
    elements.forEach(element => {
      // Set initial state if needed
      if (properties.opacity) {
        element.style.opacity = properties.opacity[0];
      }
      
      // Create the animation when element is in viewport
      const observer = new IntersectionObserver((entries) => {
        if (entries[0].isIntersecting) {
          // Import anime dynamically to avoid SSR issues
          import('animejs').then(({ default: anime }) => {
            anime({
              targets: element,
              ...properties,
              easing: animation.easing || timelineOptions.easing,
              duration: animation.duration || timelineOptions.duration,
            });
          });
          
          // Unobserve after animation starts
          observer.unobserve(element);
        }
      }, { threshold: 0.1 });
      
      observer.observe(element);
    });
  });
}

/**
 * Apply a wave animation to a group of elements
 * @param {NodeList|Array} elements - The elements to animate
 * @param {Object} options - Animation options
 */
export function createWaveAnimation(elements, options = {}) {
  const defaultOptions = {
    translateY: [-10, 0],
    duration: 1200,
    delay: anime.stagger(100, {start: 300}),
    easing: 'cubicBezier(0.42, 0, 0.58, 1)',
    loop: true,
    direction: 'alternate',
  };
  
  const animationOptions = { ...defaultOptions, ...options };
  
  // Import anime dynamically to avoid SSR issues
  import('animejs').then(({ default: anime }) => {
    anime({
      targets: elements,
      ...animationOptions,
    });
  });
}

/**
 * Create a ripple effect animation on click
 * @param {HTMLElement} element - The element to apply the ripple effect to
 */
export function createRippleEffect(element) {
  element.addEventListener('click', (e) => {
    // Create ripple element
    const ripple = document.createElement('span');
    ripple.classList.add('ripple-effect');
    
    // Position the ripple
    const rect = element.getBoundingClientRect();
    const size = Math.max(rect.width, rect.height) * 2;
    
    ripple.style.width = `${size}px`;
    ripple.style.height = `${size}px`;
    ripple.style.left = `${e.clientX - rect.left - size / 2}px`;
    ripple.style.top = `${e.clientY - rect.top - size / 2}px`;
    
    // Add ripple to element
    element.appendChild(ripple);
    
    // Import anime dynamically to avoid SSR issues
    import('animejs').then(({ default: anime }) => {
      anime({
        targets: ripple,
        scale: [0, 1],
        opacity: [0.8, 0],
        duration: 800,
        easing: 'easeOutExpo',
        complete: () => {
          ripple.remove();
        },
      });
    });
  });
}

// Card entry animation
export function cardEntryAnimation(element) {
  return animate({
    targets: element,
    opacity: [0, 1],
    translateY: [20, 0],
    scale: [0.9, 1],
    duration: 800,
    easing: 'cubicBezier(0.175, 0.885, 0.32, 1.275)'
  });
}

// Image gallery hover effect
export function imageHoverEffect(element, reverse = false) {
  return animate({
    targets: element,
    scale: reverse ? [1.05, 1] : [1, 1.05],
    boxShadow: reverse ? 
      ['0 8px 32px rgba(28, 31, 74, 0.3)', '0 8px 16px rgba(28, 31, 74, 0.2)'] : 
      ['0 8px 16px rgba(28, 31, 74, 0.2)', '0 8px 32px rgba(28, 31, 74, 0.3)'],
    duration: 400,
    easing: 'easeOutQuad'
  });
}

// Page transition effect
export function pageTransition(container) {
  const anime = animate();
  return animate({
    targets: container.querySelectorAll('.glass, .glass-sm, .glass-accent'),
    opacity: [0, 1],
    translateY: [15, 0],
    delay: anime.stagger(100, {start: 300}),
    duration: 800,
    easing: 'easeOutSine'
  });
}

// Pulsating effect for CTAs
export function pulsateEffect(element) {
  return animate({
    targets: element,
    scale: [1, 1.05, 1],
    boxShadow: [
      '0 8px 16px rgba(28, 31, 74, 0.2)',
      '0 12px 24px rgba(28, 31, 74, 0.3)',
      '0 8px 16px rgba(28, 31, 74, 0.2)'
    ],
    duration: 2000,
    easing: 'easeInOutSine',
    loop: true
  });
}

// Dreamy background flow animation
export function dreamyBackgroundFlow(element) {
  return animate({
    targets: element,
    background: [
      'linear-gradient(135deg, rgba(28, 31, 74, 0.7), rgba(42, 46, 110, 0.7))',
      'linear-gradient(225deg, rgba(28, 31, 74, 0.7), rgba(42, 46, 110, 0.7))',
      'linear-gradient(315deg, rgba(28, 31, 74, 0.7), rgba(42, 46, 110, 0.7))',
      'linear-gradient(45deg, rgba(28, 31, 74, 0.7), rgba(42, 46, 110, 0.7))'
    ],
    duration: 8000,
    easing: 'easeInOutQuad',
    loop: true
  });
}

// Text reveal animation
export function textReveal(element) {
  const textWrapper = element;
  textWrapper.innerHTML = textWrapper.textContent.replace(
    /\S/g,
    "<span class='letter'>$&</span>"
  );
  
  const anime = animate();
  return animate({
    targets: textWrapper.querySelectorAll('.letter'),
    opacity: [0, 1],
    translateY: [20, 0],
    duration: 1000,
    delay: anime.stagger(30)
  });
}

// Slide in from left animation
export function slideInFromLeft(element) {
  return animate({
    targets: element,
    translateX: [-50, 0],
    opacity: [0, 1],
    duration: 800,
    easing: 'easeOutQuad'
  });
}

// Slide in from right animation
export function slideInFromRight(element) {
  return animate({
    targets: element,
    translateX: [50, 0],
    opacity: [0, 1],
    duration: 800,
    easing: 'easeOutQuad'
  });
}

// Slide in from bottom animation
export function slideInFromBottom(element, delay = 0) {
  return animate({
    targets: element,
    translateY: [50, 0],
    opacity: [0, 1],
    duration: 800,
    delay: delay,
    easing: 'easeOutQuad'
  });
}

// Scroll based animation observer
export function createScrollObserver(selector, animationFn, options = {}) {
  const elements = document.querySelectorAll(selector);
  
  const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        animationFn(entry.target, options);
        if (options.once) {
          observer.unobserve(entry.target);
        }
      }
    });
  }, {
    threshold: options.threshold || 0.1,
    rootMargin: options.rootMargin || '0px'
  });
  
  elements.forEach(element => observer.observe(element));
  return observer;
}

// Staggered elements animation on scroll
export function staggeredElementsOnScroll(elements, options = {}) {
  const anime = animate();
  return animate({
    targets: elements,
    translateY: [options.distance || 30, 0],
    opacity: [0, 1],
    duration: options.duration || 800,
    delay: anime.stagger(options.staggerDelay || 150),
    easing: options.easing || 'easeOutQuad'
  });
}

// Glow effect animation
export function glowEffect(element) {
  return animate({
    targets: element,
    boxShadow: [
      '0 0 10px rgba(163, 241, 255, 0.0)',
      '0 0 20px rgba(163, 241, 255, 0.6)',
      '0 0 10px rgba(163, 241, 255, 0.0)'
    ],
    duration: 2000,
    easing: 'easeInOutSine',
    loop: true
  });
} 