import { useFetchApi } from '@/composables/useFetchApi.js'

export const usePosts = () => {
  const { fetchApi } = useFetchApi()

  const fetchPosts = (/*page = 1, limit = 10*/) => {
    // return fetchApi(`/posts?page=${page}&limit=${limit}`, { immediate: false }).json()
    return fetchApi(`/posts`, { immediate: true }).json()
  }
  const fetchPostBySlug = (slug) => {
    return fetchApi(`/posts/slug/${slug}`, { immediate: true }).json()
  }
  return { fetchPosts, fetchPostBySlug }
}
