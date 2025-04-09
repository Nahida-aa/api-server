# 使用 Python 内置的 http.server 开启一个简单的 HTTP 服务
from http.server import BaseHTTPRequestHandler,SimpleHTTPRequestHandler, HTTPServer
import json

def api_login(api_handler:BaseHTTPRequestHandler):
    """
    登录接口
    :param api_handler: 请求处理类
    :return: None
    """
    # 获取请求体长度
    content_length = int(api_handler.headers['Content-Length'])
    # 读取请求体
    post_data = api_handler.rfile.read(content_length)
    # 将请求体解析为 JSON
    try:
        data = json.loads(post_data) # <class 'dict'> any
        # print(type(data)) # []
        username = data.get("username")
        password = data.get("password")
        
        # 简单的登录逻辑（仅供示例）
        if username == "aa" and password == "aa":
            token = "mock-token";
            expires = 3600;
            response = {"token": token, "expires": expires}
            api_handler.send_response(200)
        else:
            response = {"msg": "密码或用户名错误"}
            api_handler.send_response(401)
    except json.JSONDecodeError:
        response = {"msg": "输入格式错误"}
        api_handler.send_response(400)
    
    # 设置响应头
    api_handler.send_header("Content-Type", "application/json")
    api_handler.end_headers()
    # 返回响应
    api_handler.wfile.write(json.dumps(response).encode("utf-8"))

# 自定义请求处理程序
class SimpleAPIHandler(BaseHTTPRequestHandler):
    def do_POST(self):
        # 解析路径
        if self.path == "/api/login":
            api_login(self)
        else:
            # 如果路径不匹配
            self.send_response(404)
            self.end_headers()
            self.wfile.write(b"Not Found")

    def do_GET(self):
        # 处理其他 GET 请求
        self.send_response(404)
        self.end_headers()
        self.wfile.write(b"Not Found")

# 定义主机和端口
host = "localhost"
port = 8000

# 创建 HTTP 服务
server = HTTPServer((host, port), SimpleAPIHandler)
print(f"Serving HTTP on {host}:{port}")

# 启动服务
server.serve_forever()