import axios from "axios"
import qs from "qs"

axios.defaults.baseURL = "http://localhost:8000/api"

const requestInterceptor = function(config) {
  // 如果是get请求会将请求参数拼接到url后面
  if (config.method === "get") {
    config.url = `${config.url}?${qs.stringify(config.params)}`
    config.params = undefined // eslint-disable-line no-undefined
  } else {
    config.data = qs.stringify(config.data)
  }
  return config
}

const responseInterceptor = ({ data: res }) =>
  res.code === 200 ? res.data : Promise.reject(res)

axios.interceptors.response.use(responseInterceptor)
axios.interceptors.request.use(requestInterceptor)
