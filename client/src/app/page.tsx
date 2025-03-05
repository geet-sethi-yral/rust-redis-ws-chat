"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";

import { Button, Input, Separator } from "@/components/ui";
import { generateRoomId } from "@/lib/utils";

export default function Page() {
  const router = useRouter();
  const [roomId, setRoomId] = useState("");

  const handleJoinRoom = () => {
    if (roomId) {
      router.push(`/room/${roomId}`);
    }
  };

  const handleCreateRoom = () => {
    const newRoomId = generateRoomId();
    router.push(`/room/${newRoomId}`);
  };

  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24 bg-background">
      <div className="w-full max-w-md p-8 space-y-8 bg-accent rounded-lg shadow-md">
        <h1 className="text-3xl font-bold text-center text-accent-foreground">
          Chat Room
        </h1>
        <p className="text-center text-accent-foreground">
          Join an existing room or create a new one
        </p>

        <div className="mt-6 space-y-4">
          <div className="flex flex-col space-y-2">
            <label
              htmlFor="room-id"
              className="text-sm font-medium text-accent-foreground"
            >
              Room ID
            </label>
            <Input
              id="room-id"
              type="text"
              value={roomId}
              onChange={(e) => setRoomId(e.target.value)}
              placeholder="Enter room ID"
              className="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div className="flex flex-col">
            <Button
              onClick={handleJoinRoom}
              className="flex-1 px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
            >
              Join a Room
            </Button>
            <Separator className="my-4 bg-accent-foreground/50" />
            <Button
              onClick={handleCreateRoom}
              className="flex-1 px-4 py-2 text-white bg-green-600 rounded-md hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2"
            >
              Create a Room
            </Button>
          </div>
        </div>
      </div>
    </main>
  );
}
