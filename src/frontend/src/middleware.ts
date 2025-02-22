// src/middleware.ts
import { defineMiddleware, sequence } from "astro:middleware";

import { PUBLIC_ROUTES } from "./const";

const AuthMiddleware = defineMiddleware(async (context, next) => {
  const token: any =
    context.request.headers.get("authorization") ||
    context.cookies.get("auth-token");

  let exists = true;
  // try {
  const existsRes = await fetch(`http://hackaton_backend:6004/auth/exists`, {
    method: "GET",
  });
  console.log("existsRes ", existsRes);
  if (existsRes.ok) {
    exists = (await existsRes.json()).data;
  }
  // } catch {}

  if (!token) {
    if (PUBLIC_ROUTES.includes(context.url.pathname)) {
      return next();
    }

    if (exists) return Response.redirect(new URL("/login", context.url));
    else return Response.redirect(new URL("/signup", context.url));
  }

    const validateRes = await fetch(
        `http://hackaton_backend:6004/auth/validate`, // Use environment variable
        {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                "Authorization": `Bearer ${token.value}`
            }
        }
    );



    if (validateRes.ok) {
        const validateResult = await validateRes.json();
        context.locals.user = validateResult.data;
        return next();
    } else {
        if (exists)
            return Response.redirect(new URL('/login', context.url));
        else
            return Response.redirect(new URL('/signup', context.url));
    }
});

export const onRequest = sequence(AuthMiddleware);
