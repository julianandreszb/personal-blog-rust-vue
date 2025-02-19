import { useFetchApi } from '@/composables/useFetchApi.js'

export const usePosts = () => {
  const { fetchApi } = useFetchApi()

  const fetchPosts = (/*page = 1, limit = 10*/) => {
    // return fetchApi(`/posts?page=${page}&limit=${limit}`, { immediate: false }).json()
    return fetchApi(`/posts`, { immediate: false }).json()
  }
  return { fetchPosts }
}
