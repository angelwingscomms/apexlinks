#!/usr/bin/env node

/**
 * PWA Validation Script
 * 
 * This script validates:
 * 1. Manifest.json completeness
 * 2. Icon set completeness
 * 3. Service worker presence
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const staticDir = path.join(__dirname, '../static');

console.log('🔍 PWA Validation Script');
console.log('=======================\n');

// 1. Check manifest.json
const manifestPath = path.join(staticDir, 'manifest.json');
let manifestValid = true;

console.log('Checking manifest.json...');
if (!fs.existsSync(manifestPath)) {
  console.error('❌ manifest.json not found!');
  manifestValid = false;
} else {
  try {
    const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
    const requiredFields = [
      'name', 'short_name', 'start_url', 'display', 
      'background_color', 'theme_color', 'icons'
    ];
    
    for (const field of requiredFields) {
      if (!manifest[field]) {
        console.error(`❌ Missing required field in manifest: ${field}`);
        manifestValid = false;
      }
    }
    
    if (manifestValid) {
      console.log('✅ manifest.json contains all required fields');
    }
    
    // Check icons array
    if (manifest.icons && Array.isArray(manifest.icons)) {
      console.log(`Found ${manifest.icons.length} icons in manifest`);
      
      // Check for minimum required sizes
      const requiredSizes = ['192x192', '512x512'];
      for (const size of requiredSizes) {
        if (!manifest.icons.some(icon => icon.sizes === size)) {
          console.error(`❌ Missing icon with size ${size} in manifest`);
          manifestValid = false;
        }
      }
      
      // Check if maskable icons are present
      if (!manifest.icons.some(icon => icon.purpose && icon.purpose.includes('maskable'))) {
        console.warn('⚠️ No maskable icons found in manifest');
      }
    } else {
      console.error('❌ No icons array in manifest or it is not an array');
      manifestValid = false;
    }
  } catch (e) {
    console.error('❌ Error parsing manifest.json:', e.message);
    manifestValid = false;
  }
}

// 2. Check icon files
const iconsDir = path.join(staticDir, 'icons');
let iconsValid = true;

console.log('\nChecking icon files...');
if (!fs.existsSync(iconsDir)) {
  console.error('❌ icons directory not found!');
  iconsValid = false;
} else {
  const iconFiles = fs.readdirSync(iconsDir);
  console.log(`Found ${iconFiles.length} icon files`);
  
  const requiredIcons = [
    'icon-72x72.png',
    'icon-96x96.png',
    'icon-128x128.png',
    'icon-144x144.png',
    'icon-152x152.png',
    'icon-192x192.png',
    'icon-384x384.png',
    'icon-512x512.png',
    'maskable-icon-192x192.png',
    'maskable-icon-512x512.png'
  ];
  
  for (const icon of requiredIcons) {
    if (!iconFiles.includes(icon)) {
      console.error(`❌ Missing icon file: ${icon}`);
      iconsValid = false;
    }
  }
  
  if (iconsValid) {
    console.log('✅ All required icon files present');
  }
}

// 3. Check service worker
const swPath = path.join(staticDir, 'service-worker.js');
let swValid = true;

console.log('\nChecking service worker...');
if (!fs.existsSync(swPath)) {
  console.error('❌ service-worker.js not found!');
  swValid = false;
} else {
  const swContent = fs.readFileSync(swPath, 'utf8');
  if (swContent.length < 100) {
    console.warn('⚠️ service-worker.js seems too small, might be incomplete');
  } else {
    console.log('✅ service-worker.js present');
  }
}

// 4. Check for Digital Asset Links file
const assetLinksPath = path.join(staticDir, '.well-known/assetlinks.json');
let assetLinksValid = true;

console.log('\nChecking Digital Asset Links...');
if (!fs.existsSync(assetLinksPath)) {
  console.error('❌ .well-known/assetlinks.json not found!');
  assetLinksValid = false;
} else {
  try {
    const assetLinks = JSON.parse(fs.readFileSync(assetLinksPath, 'utf8'));
    if (Array.isArray(assetLinks) && assetLinks.length > 0) {
      const fingerprint = assetLinks[0]?.target?.sha256_cert_fingerprints?.[0];
      if (fingerprint === 'YOUR_SHA256_FINGERPRINT_HERE') {
        console.warn('⚠️ SHA256 fingerprint is still set to placeholder value');
      } else {
        console.log('✅ Digital Asset Links file present and valid');
      }
    } else {
      console.error('❌ Invalid Digital Asset Links format');
      assetLinksValid = false;
    }
  } catch (e) {
    console.error('❌ Error parsing assetlinks.json:', e.message);
    assetLinksValid = false;
  }
}

// Summary
console.log('\n=======================');
console.log('PWA Validation Summary:');
console.log(`Manifest: ${manifestValid ? '✅ Valid' : '❌ Invalid'}`);
console.log(`Icons: ${iconsValid ? '✅ Complete' : '❌ Incomplete'}`);
console.log(`Service Worker: ${swValid ? '✅ Present' : '❌ Missing'}`);
console.log(`Asset Links: ${assetLinksValid ? '✅ Valid' : '❌ Invalid or Missing'}`);
console.log('=======================');

// Exit with error code if any validation failed
if (!manifestValid || !iconsValid || !swValid || !assetLinksValid) {
  process.exit(1);
}

console.log('\n✅ PWA validation completed successfully'); 