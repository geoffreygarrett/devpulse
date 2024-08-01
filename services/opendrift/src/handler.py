from datetime import datetime
from opendrift.models.oceandrift import OceanDrift
from typing import Dict

class SimulationHandler:
    def __init__(self):
        self.models = {
            "OceanDrift": OceanDrift,
            # Add other models as needed
        }

    async def run_simulation(self, model_name: str, lon: float, lat: float, radius: float,
                             start_time: str, end_time: str, duration_hours: int) -> Dict:
        model_class = self.models.get(model_name)
        if not model_class:
            raise ValueError("Model not supported")

        start_time = datetime.fromisoformat(start_time)
        end_time = datetime.fromisoformat(end_time)

        model = model_class()
        model.seed_elements(lon=lon, lat=lat, radius=radius, time=start_time)
        model.run(duration=duration_hours * 3600)

        return {"status": "success", "result_file": model.get_output_path()}
