# AngelWings Comprehensive College Website

A stunning, interactive website for AngelWings Comprehensive College - where creativity meets STEM education. Built with SvelteKit, Tailwind CSS v4, Flowbite, and featuring an elegant gold design with sophisticated glassmorphism effects.

## 🚀 Features

### Design & Aesthetics
- **Elegant Glassmorphism**: Refined glass-like UI elements on warm gold backgrounds
- **Premium Gold Theme**: Sophisticated cream/gold palette with deep blue accents creating a luxurious feel
- **Responsive Layout**: Fully responsive design that works on all devices
- **Custom Animations**: Subtle, refined animations using Tailwind CSS Motion (Rombo)

### Interactive Components
- **Particle Background**: Animated particle system with connecting lines
- **STEM Explorer**: Interactive component to explore different STEM fields
- **Creative Showcase**: Rotating carousel of student projects and achievements
- **Student Journey Timeline**: Interactive timeline showing educational progression
- **Floating Action Button**: Quick access to applications and contact

### Content Sections
- **Hero Section**: Compelling introduction with animated elements
- **About Section**: School philosophy and educational approach
- **STEM Programs**: Interactive exploration of 6 STEM areas
- **Facilities**: Showcase of world-class learning environments
- **Student Projects**: Real student innovations and achievements
- **Educational Journey**: Timeline from discovery to innovation
- **Statistics**: Key metrics and achievements
- **Contact Form**: Interactive contact form with program selection

## 🛠 Technology Stack

- **Framework**: SvelteKit 2.0 (Static Site Generation)
- **Styling**: Tailwind CSS 4.0
- **UI Components**: Flowbite 5.0
- **Animations**: Tailwind CSS Motion (Rombo)
- **Language**: TypeScript
- **Build Tool**: Vite
- **Package Manager**: npm

## 📦 Dependencies

### Core Dependencies
- `svelte`: ^5.0.0
- `@sveltejs/kit`: ^2.16.0
- `tailwindcss`: ^4.0.0
- `flowbite`: ^5.0.37
- `tailwindcss-motion`: ^1.1.0
- `axios`: ^1.9.0

### Development Dependencies
- `@sveltejs/adapter-static`: ^3.0.8
- `@tailwindcss/forms`: ^0.5.9
- `@tailwindcss/typography`: ^0.5.15
- `typescript`: ^5.0.0
- `vite`: ^6.2.6

## 🎨 Design System

### Color Palette
- **Primary**: Deep Blue (#3238A6) - Used for main elements and emphasis
- **Secondary**: Bright Blue (#049DD9) - Supporting elements
- **Tertiary**: Light Blue (#04B2D9) - Accent elements and highlights
- **Accent**: Gold/Amber (#D9933D) - Call to action and important elements
- **Background**: Warm Golds (#fbf7f0, #f7f2e6, #efe8d6) - Layered backgrounds

### Custom Utilities
- `.glass`: Standard glassmorphic effect (white on gold)
- `.glass-blue`: Blue-tinted glassmorphism
- `.glass-gold`: Gold-tinted glassmorphism
- `.glass-lg`: Large glassmorphic elements
- `.golden-bg`: Warm gold background
- `.golden-gradient`: Animated gold gradient background
- `.glow-gold`: Gold glow effect
- `.border-glow-blue`: Blue border glow effect
- `.border-glow-gold`: Gold border glow effect

### Animations
- `animate-golden-shift`: Shifting gold background gradients
- `animate-glow`: Gold/blue alternating glow effect
- `animate-pulse-blue`: Blue pulsing animation
- `animate-pulse-gold`: Gold pulsing animation
- Motion presets from Tailwind CSS Motion

## 🏗 Project Structure

```
aw/
├── src/
│   ├── lib/
│   │   ├── STEMExplorer.svelte          # Interactive STEM field explorer
│   │   ├── ParticleBackground.svelte     # Animated particle system
│   │   ├── CreativeShowcase.svelte      # Student project carousel
│   │   ├── StudentJourney.svelte        # Educational timeline
│   │   └── FloatingActionButton.svelte  # Quick action menu
│   ├── routes/
│   │   ├── +layout.svelte               # Main layout with theme
│   │   └── +page.svelte                 # Homepage with all sections
│   ├── app.css                          # Global styles and utilities
│   └── app.html                         # HTML template
├── static/
│   └── images/                          # Project images
└── package.json                         # Dependencies and scripts
```

## 🚀 Getting Started

### Prerequisites
- Node.js 18+ 
- npm or yarn

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd aw
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Start development server**
   ```bash
   npm run dev
   ```

4. **Open in browser**
   Navigate to `http://localhost:5173`

### Build for Production

```bash
npm run build
```

The static site will be generated in the `build/` directory.

## 🎯 Educational Focus

AngelWings Comprehensive College emphasizes:

### STEM Areas Covered
1. **Computer Science** - Programming, AI, Software Development
2. **Engineering** - Mechanical, Electrical, Civil, Robotics
3. **Mathematics** - Pure Math, Applied Math, Statistics
4. **Physics** - Quantum Mechanics, Astrophysics, Applied Physics
5. **Chemistry** - Organic, Biochemistry, Materials Science
6. **Biology** - Genetics, Biotechnology, Environmental Science

### Educational Philosophy
- **Creativity First**: Encouraging creative problem-solving
- **Hands-on Learning**: Project-based education
- **Individual Discovery**: Helping students find their passion
- **Real-world Applications**: Connecting theory to practice

## 🌟 Key Features Showcase

### Interactive STEM Explorer
- Click on any STEM area to explore in detail
- Tabbed interface showing skills, projects, and careers
- Smooth animations and transitions
- Comprehensive information for each field

### Student Project Carousel
- Auto-rotating showcase of real student innovations
- Detailed project information with technologies used
- Achievement highlights and recognition
- Interactive navigation controls

### Educational Journey Timeline
- Four-phase progression from discovery to innovation
- Interactive timeline with progress indicators
- Detailed activities for each educational phase
- Auto-advancing with manual controls

### Particle Animation System
- Canvas-based particle system
- Dynamic connections between particles
- Performance-optimized rendering
- Responsive to screen size

## 📱 Responsive Design

The website is fully responsive with breakpoints:
- **Mobile**: < 640px
- **Tablet**: 640px - 1024px  
- **Desktop**: 1024px+
- **Large Desktop**: 1280px+

## ♿ Accessibility

- Semantic HTML structure
- ARIA labels and roles
- Keyboard navigation support
- High contrast color scheme
- Screen reader friendly

## 🔧 Customization

### Modifying Colors
Edit the `@theme` section in `app.css` to change the color scheme:

```css
@theme {
  --color-primary: #3238A6;    /* Change primary color */
  --color-secondary: #049DD9;  /* Change secondary color */
  /* ... other colors */
}
```

### Adding New Animations
Add custom animations in the `@theme` section in `app.css`:

```css
@theme {
  /* ... */
  @keyframes custom-animation {
    0% { /* properties */ }
    100% { /* properties */ }
  }
}
```

### Modifying Content
- Edit STEM areas in `+page.svelte`
- Update student projects in `CreativeShowcase.svelte`
- Modify journey steps in `StudentJourney.svelte`

## 🚀 Deployment

### Static Hosting (Recommended)
The site is configured for static generation and can be deployed to:
- Vercel
- Netlify
- GitHub Pages
- Any static hosting service

### Build Command
```bash
npm run build
```

## 📄 License

This project is created for AngelWings Comprehensive College. All rights reserved.

## 🤝 Contributing

This is a custom website for AngelWings Comprehensive College. For modifications or updates, please contact the development team.

## 📞 Support

For technical support or questions about the website:
- Email: tech@angelwings.edu
- Phone: (555) 123-STEM

---

**Built with ❤️ for AngelWings Comprehensive College - Inspiring creativity through premium education**
