import { Card, CardTitle, CardDescription } from "../ui/card";
import RenderLineChart from "../ui/charts";

export function CardDemo() {
  return (
    <Card className="max-w-xl max-h-[800px] !bg-black justify-center">
      <CardTitle>Name</CardTitle>
      <CardDescription className="mb-4">
        A card that showcases a set of tools that you use to create your
        product.
      </CardDescription>
      <RenderLineChart />
    </Card>
  );
}
