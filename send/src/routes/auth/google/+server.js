export function GET() {
  // Google OAuth configuration
  const clientId = process.env.GOOGLE_CLIENT_ID;
  const redirectUri = `${process.env.PUBLIC_BASE_URL}/auth/callback`;
  const scope = 'email profile';
  
  // Build the OAuth URL
  const authUrl = new URL('https://accounts.google.com/o/oauth2/v2/auth');
  authUrl.searchParams.append('client_id', clientId);
  authUrl.searchParams.append('redirect_uri', redirectUri);
  authUrl.searchParams.append('response_type', 'code');
  authUrl.searchParams.append('scope', scope);
  authUrl.searchParams.append('access_type', 'offline');
  authUrl.searchParams.append('prompt', 'consent');
  
  // Redirect to Google OAuth
  return new Response(null, {
    status: 302,
    headers: {
      'Location': authUrl.toString()
    }
  });
} 