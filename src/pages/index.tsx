import { invoke } from "@tauri-apps/api/tauri";
import React from "react";
import { Box, Button, VStack } from "@chakra-ui/react";
import { Line } from "react-chartjs-2";
import {
  CategoryScale,
  Chart,
  Legend,
  LinearScale,
  LineElement,
  PointElement,
  Title,
  Tooltip,
} from "chart.js";
import { useQuery } from "react-query";

interface Data {
  data: HeartData[];
}
interface HeartData {
  date: string;
  heart_rate: number;
}

interface SleepData {
  data: Sleep[];
}

interface Sleep {
  sleep_date: string;
  efficiency: number;
  end_time: string;
  rem: number;
  light: number;
  deep: number;
  wake: number;
}

Chart.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend);

export default function Home() {
  React.useEffect(() => {
    const queryParameters = new URLSearchParams(window.location.search);
    const fitbit_code = queryParameters.get("code");
    if (fitbit_code) {
      alert("INVOKING FITBIT TOKEN");
      invoke("get_fitbit_token", { fitbitCode: fitbit_code });
    }
  }, []);

  function getSleepData() {
    return invoke("get_sleep_data").then((data: SleepData | unknown) => {
      if (data) {
        return handleSleepData(data as SleepData);
      }
    });
  }

  function getHeartData() {
    return invoke("get_heart_data").then((data: Data | unknown) => {
      if (data) {
        return handleHeartData(data as Data);
      }
    });
  }

  function handleHeartData(data: Data) {
    let { data: heartResponse } = data;
    const labels = heartResponse.map((val) => val.date);
    const values = heartResponse.map((val) => val.heart_rate);
    const chartData = {
      labels,
      datasets: [
        {
          label: "Heart Rate",
          data: values,
          fill: false,
          backgroundColor: "rgb(255, 99, 132)",
          borderColor: "rgba(255, 99, 132, 0.2)",
        },
      ],
    };
    return chartData;
  }

  function handleSleepData(data: SleepData) {
    let { data: sleepResponse } = data;
    const rem = sleepResponse.map((val) => ({
      x: val.end_time.slice(5, -7).replace("T", " "),
      y: val.rem,
    }));
    const chartData = {
      datasets: [
        {
          label: "REM Sleep (Minutes)",
          data: rem,
          fill: false,
          backgroundColor: "rgb(132, 99, 255)",
          borderColor: "rgba(132, 99, 255, 0.3)",
        },
      ],
    };

    const options = {
      scales: {
        x: {
          ticks: {
            font: {
              size: 14,
            },
          },
        },
      },
    };
    return { chartData, options };
  }

  const heartQuery = useQuery("heartData", () => getHeartData(), {
    refetchOnWindowFocus: false,
    refetchOnMount: false,
    refetchOnReconnect: false,
  });

  const sleepQuery = useQuery("sleepData", () => getSleepData(), {
    refetchOnWindowFocus: false,
    refetchOnMount: false,
    refetchOnReconnect: false,
  });

  if (heartQuery.isLoading || sleepQuery.isLoading) {
    return <Box>Loading...</Box>;
  }
  let heartData = heartQuery.data;
  let sleepData = sleepQuery.data?.chartData;
  let sleepOptions = sleepQuery.data?.options;

  return (
    <VStack w="100%">
      <VStack w="500px" rowGap="20px">
        {typeof heartData !== undefined && <Line data={heartData!} />}
        {typeof sleepData !== undefined && <Line data={sleepData!} options={sleepOptions} />}
      </VStack>
    </VStack>
  );
}
