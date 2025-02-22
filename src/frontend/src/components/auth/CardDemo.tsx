import { Card, CardTitle, CardDescription } from "../ui/card";
import RenderLineChart from "../ui/charts";

export function CardDemo() {
  return (
    <Card className="w-1/3 justify-center">
      <div className="ml-3">
        <CardTitle className="-mb-3">Name</CardTitle>
        <CardDescription className="mb-4">
          A card that showcases a set of tools that you use to create your
          product.
        </CardDescription>
      </div>
      <div className="w-full h-44">
        <RenderLineChart />
      </div>
    </Card>
  );
}
