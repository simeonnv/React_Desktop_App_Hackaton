"use client";
import { TrendingUp } from "lucide-react";
import { CartesianGrid, Line, LineChart, XAxis, YAxis } from "recharts";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from "@/components/ui/chart";

type ChartData = {
  time: number;
  usage: number;
};

type Props = {
  graph_data: ChartData[];
  max_usage: number;
  title: string;
  description: string;
};

const chartConfig = {
  usage: {
    label: "Usage",
    color: "hsl(var(--chart-1))",
  },
} satisfies ChartConfig;

export default function Charts({ graph_data, max_usage, title, description }: Props) {
  // Format the data to ensure it matches what Recharts expects
  const formattedData = graph_data.map((item) => ({
    time: item.time,
    usage: item.usage,
  }));

  // Calculate min and max usage values for dynamic Y-axis scaling with padding
  const usageValues = graph_data.map((item) => item.usage);
  const minUsage = Math.min(...usageValues);
  const maxUsage = Math.max(...usageValues);
  const range = maxUsage - minUsage || 1; // Avoid division by 0
  const padding = range * 0.2; // 20% padding for visibility
  const yMin = Math.max(0, minUsage - padding); // Prevent negative values
  const yMax = Math.min(max_usage, maxUsage + padding); // Respect max_usage cap

  return (
    <Card>
      <CardHeader>
        <CardTitle>{title}</CardTitle>
        <CardDescription>{description}</CardDescription>
      </CardHeader>
      <CardContent>
        <ChartContainer config={chartConfig}>
          <LineChart
            accessibilityLayer
            data={formattedData}
            margin={{
              top: 20,
              right: 20,
              bottom: 20,
              left: 20,
            }}
          >
            <CartesianGrid vertical={false} />
            <XAxis
              dataKey="time"
              tickLine={false}
              axisLine={false}
              tickMargin={10}
              tickFormatter={(value) => value.toFixed(0)} // Format time as integer
            />
            <YAxis
              domain={[yMin, yMax]}
              tickLine={false}
              axisLine={false}
              tickMargin={10}
              tickFormatter={(value) => value.toFixed(2)} // 2 decimal places for usage
            />
            <ChartTooltip
              cursor={false}
              content={<ChartTooltipContent indicator="dot" hideLabel />}
            />
            <Line
              dataKey="usage"
              type="linear" // Fixed linear interpolation
              stroke="var(--color-usage)"
              strokeWidth={2}
              dot={false}
              animationDuration={0} // Disable animations
            />
          </LineChart>
        </ChartContainer>
      </CardContent>
      <CardFooter className="flex-col items-start gap-2 text-sm">
        {/* <div className="flex gap-2 font-medium leading-none">
          Trending up by 5.2% this month <TrendingUp className="h-4 w-4" />
        </div>
        <div className="leading-none text-muted-foreground">
          Showing total usage over time
        </div> */}
      </CardFooter>
    </Card>
  );
}