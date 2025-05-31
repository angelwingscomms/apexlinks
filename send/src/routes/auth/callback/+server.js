import { json } from '@sveltejs/kit';
import axios from 'axios';

export async function GET({ url }) {
  const code = url.searchParams.get('code');
  
  if (!code) {
    return new Response(JSON.stringify({ error: 'No authorization code provided' }), {
      status: 400,
      headers: { 'Content-Type': 'application/json' }
    });
  }
  
  try {
    // Google OAuth token exchange
    const tokenResponse = await axios.post('https://oauth2.googleapis.com/token', {
      code,
      client_id: process.env.GOOGLE_CLIENT_ID,
      client_secret: process.env.GOOGLE_CLIENT_SECRET,
      redirect_uri: `${process.env.PUBLIC_BASE_URL}/auth/callback`,
      grant_type: 'authorization_code'
    });
    
    const { access_token, id_token } = tokenResponse.data;
    
    // Get user info
    const userInfoResponse = await axios.get('https://www.googleapis.com/oauth2/v2/userinfo', {
      headers: { Authorization: `Bearer ${access_token}` }
    });
    
    const { email, name, picture } = userInfoResponse.data;
    
    // Create a session token with user info (in production, this would be signed with JWT)
    const sessionToken = Buffer.from(JSON.stringify({ email, name, picture })).toString('base64');
    
    // Redirect to auth page with token
    return new Response(null, {
      status: 302,
      headers: {
        'Location': `/auth?token=${sessionToken}`
      }
    });
  } catch (error) {
    console.error('OAuth callback error:', error);
    
    return new Response(JSON.stringify({ error: 'Authentication failed' }), {
      status: 500,
      headers: { 'Content-Type': 'application/json' }
    });
  }
} 