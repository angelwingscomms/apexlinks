import { onCLS, onINP, onLCP, onFCP, onTTFB } from 'web-vitals';

type MetricName = 'CLS' | 'INP' | 'LCP' | 'FCP' | 'TTFB';

interface MetricData {
  name: MetricName;
  value: number;
  id: string;
  navigationType: string | undefined;
}

function sendToAnalytics(metric: MetricData): void {
  // This function could send metrics to an analytics endpoint
  // For now, we'll just log them to console in development
  if (process.env.NODE_ENV === 'development') {
    console.log(`Web Vitals: ${metric.name}`, metric);
  }
  
  // In a production environment, you'd send this data to your analytics service
  // Example:
  // fetch('/api/analytics', {
  //   method: 'POST',
  //   body: JSON.stringify(metric),
  //   headers: { 'Content-Type': 'application/json' }
  // });
}

export function initWebVitals(): void {
  // Core Web Vitals
  onCLS((metric) => {
    sendToAnalytics({
      name: 'CLS',
      value: metric.value,
      id: metric.id,
      navigationType: metric.navigationType
    });
  });
  
  onINP((metric) => {
    sendToAnalytics({
      name: 'INP',
      value: metric.value,
      id: metric.id,
      navigationType: metric.navigationType
    });
  });
  
  onLCP((metric) => {
    sendToAnalytics({
      name: 'LCP',
      value: metric.value,
      id: metric.id,
      navigationType: metric.navigationType
    });
  });
  
  // Additional metrics
  onFCP((metric) => {
    sendToAnalytics({
      name: 'FCP',
      value: metric.value,
      id: metric.id,
      navigationType: metric.navigationType
    });
  });
  
  onTTFB((metric) => {
    sendToAnalytics({
      name: 'TTFB',
      value: metric.value,
      id: metric.id,
      navigationType: metric.navigationType
    });
  });
} 