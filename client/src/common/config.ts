declare global {
  interface Window {
    API_ENDPOINT: string
    ENV: string
  }
}

export const Config = {
  API_ENDPOINT: window.API_ENDPOINT || ``,
  ENV: window.ENV || 'develop',
}
