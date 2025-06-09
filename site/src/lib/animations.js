import { animate } from 'animejs';

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