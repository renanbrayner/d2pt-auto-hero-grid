import { useState } from "react";
import { useToast } from "@/hooks/use-toast";
import { invoke } from "@tauri-apps/api/core";
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from "@/components/ui/carousel";
import { Button } from "@/components/ui/button";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const { toast } = useToast();

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  function getFilePath() {
    invoke("get_hero_grid_path")
      .then((path) => {
        console.log(path);
      })
      .catch((err) => {
        toast({
          title: "Error finding path",
          description: err,
          variant: "destructive",
        });
      });
  }

  return (
    <main className="flex flex-col items-center justify-center h-screen">
      <Button onClick={getFilePath}>Foo</Button>
      <Carousel>
        <CarouselContent>
          <CarouselItem>1</CarouselItem>
          <CarouselItem>2</CarouselItem>
          <CarouselItem>3</CarouselItem>
        </CarouselContent>
        <CarouselPrevious />
        <CarouselNext />
      </Carousel>
    </main>
  );
}

export default App;
