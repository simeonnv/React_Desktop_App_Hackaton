// src/middleware.ts
import { defineMiddleware, sequence } from 'astro:middleware';

import { PUBLIC_ROUTES } from "./const";

const AuthMiddleware = defineMiddleware(async (context, next) => {

    if (PUBLIC_ROUTES.includes(context.url.pathname)) {
        return next();
    }

    const token = context.request.headers.get('authorization') ||
        context.cookies.get('auth-token');

    if (!token) {
        return Response.redirect(new URL('/signup', context.url));
    }

    const response = await fetch("http://localhost:6004/auth/validate", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            "Authorization": `Bearer ${token}`
        }
    });

    if (response.ok) {
        const result = await response.json();
        context.locals.user = result.user;
        return next();
    } else {
        return Response.redirect(new URL('/signup', context.url));
    }
});


export const onRequest = sequence(AuthMiddleware);
