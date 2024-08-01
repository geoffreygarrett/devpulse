import asyncio
import grpc
from src.proto import opendrift_pb2_grpc, opendrift_pb2
from handler import SimulationHandler


class OpenDriftSimulatorServicer(opendrift_pb2_grpc.OpenDriftSimulatorServicer):
    def __init__(self):
        self.handler = SimulationHandler()

    async def RunSimulation(self, request, context):
        result = await self.handler.run_simulation(
            request.model_name, request.lon, request.lat, request.radius,
            request.start_time, request.end_time, request.duration_hours
        )
        return opendrift_pb2.SimulationResponse(
            status=result["status"], result_file=result["result_file"]
        )


async def serve():
    server = grpc.aio.server()
    opendrift_pb2_grpc.add_OpenDriftSimulatorServicer_to_server(
        OpenDriftSimulatorServicer(), server
    )
    server.add_insecure_port('[::]:50051')
    await server.start()
    await server.wait_for_termination()


if __name__ == '__main__':
    asyncio.run(serve())
