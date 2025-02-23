import { NextRequest, NextResponse } from 'next/server';

export async function middleware(request: NextRequest) {
  const token = request.cookies.get('auth-token')?.value;
  
  // Define paths to skip middleware (e.g., signup or login pages)
  const pathname = request.nextUrl.pathname;
  console.log(pathname)
  if (pathname === '/signup' || pathname === '/login') {
    console.log("CONTINUE")
    return NextResponse.next();
  }

  console.log("TOKEN: ", token)

  if (token) {
    try {
      const tokenResponse = await fetch('http://hackaton_backend:6004/auth/validate', {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${token}`,
        },
      });

      console.log("OK ", tokenResponse.ok)

      if (tokenResponse.ok) {
        const accountResponse = await fetch('http://hackaton_backend:6004/auth/exists', {
          method: 'GET',
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });
        console.log("EXISTS: ", accountResponse.ok)
        const accountData = await accountResponse.json();
        console.log(accountData)

        if (accountResponse.ok && accountData.status) {
          return NextResponse.next();
        } else {
          return NextResponse.redirect(new URL('/signup', request.url));
        }
      } else {

        try {
          const accountResponse = await fetch('http://hackaton_backend:6004/auth/exists', {
            method: 'GET',
            headers: {
              Authorization: `Bearer ${token}`,
            },
          });
          if (accountResponse.ok) {
            let data = await accountResponse.json()
            if (data.data === "true") {
              return NextResponse.redirect(new URL('/login', request.url));
            } else {
              return NextResponse.redirect(new URL('/signup', request.url));
            }
          }
        } catch {}

        return NextResponse.redirect(new URL('/login', request.url));
      }
    } catch (error) {
      console.error('Middleware error:', error);
      return NextResponse.redirect(new URL('/login', request.url));
    }
  } else {
    return NextResponse.redirect(new URL('/login', request.url));
  }
}

export const config = {
  matcher: ['/((?!api|_next/static|_next/image|favicon.ico).*)'], // Apply to all routes except API, static files, etc.
};