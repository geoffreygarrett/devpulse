import asyncio
from src.grpc import serve as grpc_serve
from src.rest import app as rest_app
from aiohttp import web


async def start_servers():
    grpc_server = asyncio.create_task(grpc_serve())
    rest_server = web._run_app(rest_app, port=5000)
    await asyncio.gather(grpc_server, rest_server)


if __name__ == '__main__':
    asyncio.run(start_servers())
