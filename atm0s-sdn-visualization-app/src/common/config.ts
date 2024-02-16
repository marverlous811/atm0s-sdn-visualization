declare global {
  interface Window {
    API_ENDPOINT: string
    ENV: string
  }
}

export const Config = {
  API_ENDPOINT: window.API_ENDPOINT || `http://localhost:8080`,
  ENV: window.ENV || 'develop',
}
