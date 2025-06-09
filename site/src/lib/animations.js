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
      ['0 8px 32px rgba(14, 165, 233, 0.3)', '0 8px 16px rgba(14, 165, 233, 0.2)'] : 
      ['0 8px 16px rgba(14, 165, 233, 0.2)', '0 8px 32px rgba(14, 165, 233, 0.3)'],
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
      '0 8px 16px rgba(14, 165, 233, 0.2)',
      '0 12px 24px rgba(14, 165, 233, 0.3)',
      '0 8px 16px rgba(14, 165, 233, 0.2)'
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
      'linear-gradient(135deg, rgba(240, 249, 255, 0.7), rgba(186, 230, 253, 0.7))',
      'linear-gradient(225deg, rgba(240, 249, 255, 0.7), rgba(186, 230, 253, 0.7))',
      'linear-gradient(315deg, rgba(240, 249, 255, 0.7), rgba(186, 230, 253, 0.7))',
      'linear-gradient(45deg, rgba(240, 249, 255, 0.7), rgba(186, 230, 253, 0.7))'
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