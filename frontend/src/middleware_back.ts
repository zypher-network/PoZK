import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";
import { tokenKey } from "./constants/constants";

export function middleware(request: NextRequest) {
  const hasToken = request.cookies.get(tokenKey)?.value;
  if (hasToken) {
    if (request.nextUrl.pathname !== "/") {
      return NextResponse.next();
    } else {
      return NextResponse.redirect(new URL("/dashboard", request.url));
    }
  }

  // 如果 hasToken 为 false，则只允许访问 '/'
  if (!hasToken && request.nextUrl.pathname !== "/") {
    return NextResponse.redirect(new URL("/", request.url));
  }

  return NextResponse.next();
}

export const config = {
  matcher: [
    // "/",
    "/dashboard/:path",
    "/dashboard/:path/:path",
    "/rewards/:path",
    "/api/game/:path",
    "/api/dashboard/:path",
  ],
};
