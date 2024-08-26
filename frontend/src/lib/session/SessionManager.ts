"use client";
import { accountKey, tokenKey } from "@/constants/constants";

class SessionManager {
  static isCookie = false;
  static getSession() {
    if (this.isCookie) {
      const name = tokenKey;
      const cookies = document.cookie.split(";");
      for (let i = 0; i < cookies.length; i++) {
        const cookie = cookies[i].trim();
        if (cookie.startsWith(`${name}=`)) {
          return cookie.substring(name.length + 1);
        }
      }
      return undefined;
    }
    const val = localStorage.getItem(tokenKey);
    return val ?? undefined;
  }
  static setSession(sessionId: string) {
    localStorage.setItem(tokenKey, sessionId);
  }
  static deleteSession() {
    localStorage.removeItem(tokenKey);
  }

  static getAccount() {
    const val = localStorage.getItem(accountKey);
    return val ?? undefined;
  }
  static setAccount(sessionId: string) {
    localStorage.setItem(accountKey, sessionId);
  }
}

export default SessionManager;
