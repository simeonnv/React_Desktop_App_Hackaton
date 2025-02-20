// src/middleware.ts
import { defineMiddleware, sequence } from 'astro:middleware';

import { PUBLIC_ROUTES } from "./const";

const AuthMiddleware = defineMiddleware(async (context, next) => {

    const token: any = context.request.headers.get('authorization') ||
        context.cookies.get('auth-token');


    if (!token) {
        if (PUBLIC_ROUTES.includes(context.url.pathname)) {
        return next();
    }
        return Response.redirect(new URL('/signup', context.url));
    }


    const response = await fetch("http://localhost:6004/auth/validate", {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            "Authorization": `Bearer ${token.value}`
        }
    });


    if (response.ok) {
        const result = await response.json();
        context.locals.user = result.data;
        return next();
    } else {
        return Response.redirect(new URL('/signup', context.url));
    }
});


export const onRequest = sequence(AuthMiddleware);
