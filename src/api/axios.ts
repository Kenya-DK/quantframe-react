
import axios from 'axios'
// @ts-ignore no type definitions for this package
import axiosTauriAdapter from 'axios-tauri-adapter'
import { APP_NAME, PLATFORMS } from '$types/index'
import { TokenBucket } from './rateLimiter'
import { settings } from '../hooks'

const HEADERS = {
  'Content-Type': 'application/json; utf-8',
  'Accept': 'application/json',
  'Authorization': 'JWT',
  'platform': PLATFORMS.PC,
  'language': 'en',
  'auth_type': 'header',
  // "auth_type": "cookie",
}

export const axiosInstance = axios.create({
  adapter: axiosTauriAdapter,
  baseURL: 'https://api.warframe.market/v1',
  headers: HEADERS
})

axiosInstance.defaults.xsrfCookieName = APP_NAME + '_csrftoken'
axiosInstance.defaults.xsrfHeaderName = 'X-CSRFToken'

// Nice interceptor implementation: https://github.com/gitdagray/react_jwt_auth/blob/main/src/hooks/useAxiosPrivate.js

// Look into how high burst can get before we get rate limited
const rateLimiter = new TokenBucket(3, 2)

axiosInstance.interceptors.request.use(
  async config => {
    const { access_token } = await settings.get()
    if (access_token) {
      config.headers['Authorization'] = `JWT ${access_token}`
    }

    await rateLimiter.wait()

    return config
  }, (error) => Promise.reject(error)
)

// Refresh Access Token ??? idk if this is needed
// wfm.interceptors.response.use(
//   response => response,
//   async (error) => {
//     const prevRequest = error?.config;
//     if (error?.response?.status === 403 && !prevRequest?.sent) {
//       prevRequest.sent = true;
//       const newAccessToken = await refresh();
//       prevRequest.headers['Authorization'] = `Bearer ${newAccessToken}`;
//       return wfmAuth(prevRequest);
//     }
//     return Promise.reject(error);
//   }
// );