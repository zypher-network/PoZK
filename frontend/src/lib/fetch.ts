import SessionManager from "./session/SessionManager";

// utils/api.ts
const API_BASE_URL =
  process.env.API_BASE_URL || "https://miner.zypher.game/api";

interface FetchOptions extends RequestInit {
  accessToken?: string;
  useUrl?: boolean;
}

async function fetcher<T>(url: string, options: FetchOptions = {}): Promise<T> {
  const { useUrl = false, ...fetchOptions } = options;
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };
  const cookie = SessionManager.getSession();
  if (cookie) {
    headers["X-API-Key"] = cookie;
  }
  // console.log({ useUrl });
  const response = await fetch(
    `${options.useUrl ? "/api" : API_BASE_URL}${url}`,
    {
      ...fetchOptions,
      headers,
    }
  );

  if (!response.ok) {
    throw new Error(`HTTP error ${response.status}`);
  }

  return response.json();
}

async function apiGet<T>(url: string, options: FetchOptions = {}): Promise<T> {
  return fetcher<T>(url, { ...options, method: "GET" });
}

async function apiPost<T>(
  url: string,
  data: any,
  options: FetchOptions = {}
): Promise<T> {
  return fetcher<T>(url, {
    ...options,
    method: "POST",
    body: JSON.stringify(data),
  });
}

async function apiPut<T>(
  url: string,
  data: any,
  options: FetchOptions = {}
): Promise<T> {
  return fetcher<T>(url, {
    ...options,
    method: "PUT",
    body: JSON.stringify(data),
  });
}

async function apiDelete<T>(
  url: string,
  options: FetchOptions = {}
): Promise<T> {
  return fetcher<T>(url, { ...options, method: "DELETE" });
}

const api = {
  get: apiGet,
  post: apiPost,
  put: apiPut,
  delete: apiDelete,
};

export default api;
