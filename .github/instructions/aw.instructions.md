---
applyTo: 'aw/*'
---
These are the rules for the aw folder, the sveltekit site for AngelWings Comprehensive College

Always think like a professional creative designer that works in a professional highend design studio that does work for fortune 500 companies like Apple and Nike

always a lot of neumorphic styles throughout using tailwind and flowbite
always use a lotta animations and transitions using rombo for tailwind (tailwindcss-motion)
dreamy ethereal styles

create with a lot of freedom, and if you have any idea for anything that will go well with an image that doesn't exist, create the ui for the image, with the dimensions so it's like a placeholder in the ui, and make the alt of the placeholder a prompt for an image to be created with AI, and also tell me every image to be created in your response

do a lott

## IMPORTANT: Tailwind CSS v4 Configuration
This project uses Tailwind CSS v4 which has a DIFFERENT configuration approach than v3:

**NEVER create tailwind.config.js** - v4 uses `@theme` directive in CSS files instead

**Correct v4 approach:**
- Use `@import 'tailwindcss';` in CSS files
- Configure theme using `@theme { }` directive in CSS
- Define custom utilities in `@layer utilities { }`
- Define components in `@layer components { }`

**Theme variables format:**
```css
@theme {
  --color-primary: #0ea5e9;
  --color-accent: #f59e0b;
  --shadow-neumorphic: 15px 15px 30px rgba(14, 165, 233, 0.1), -15px -15px 30px rgba(255, 255, 255, 0.8);
}
```

**v3 vs v4 compatibility:**
- v3 config files (tailwind.config.js) are INCOMPATIBLE with v4
- Flowbite components need to be recreated in v4 syntax
- Custom utilities must use direct CSS properties, not Tailwind classes

## Current Theme: Ambient Dreamy Sky
colors
    sky blue (#0ea5e9) - primary
    navy blue (#1e3a8a) - secondary  
    gold (#f59e0b) - accent
    light backgrounds (#f0f9ff, #e0f2fe, #bae6fd)
    dark text (#0f172a) for contrast

## Educational Content
extracurriculars
    robotics    
    chess
    coding
        making software with AI
    making them proficientt in AI use accross various domains

the school has these classes
    foundation
    prenursery
    nursery 1
    nursery 2
    grade 1
    grade 2
    grade 3
    grade 4
    grade 5


