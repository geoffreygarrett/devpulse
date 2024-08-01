from aiohttp import web
from handler import SimulationHandler

handler = SimulationHandler()


async def run_simulation(request):
    data = await request.json()
    result = await handler.run_simulation(
        data['model_name'], data['lon'], data['lat'], data['radius'],
        data['start_time'], data['end_time'], data['duration_hours']
    )
    return web.json_response(result)


app = web.Application()
app.router.add_post('/simulate', run_simulation)

if __name__ == '__main__':
    web.run_app(app, port=5000)
