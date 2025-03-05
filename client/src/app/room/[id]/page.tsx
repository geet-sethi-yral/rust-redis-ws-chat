"use client";

import { useEffect, useState } from "react";
import { io, Socket } from "socket.io-client";

import { Button, Input } from "@/components/ui";
import { useParams } from "next/navigation";
import { cn, colorForName } from "@/lib/utils";

interface Message {
  text: string;
  user: string;
  date: string;
}

export default function ChatRoom() {
  const { id: roomId } = useParams<{ id: string }>();
  const [socket, setSocket] = useState<Socket | null>(null);
  const [message, setMessage] = useState("");
  const [messages, setMessages] = useState<Message[]>([]);

  useEffect(() => {
    const socket = io("ws://localhost:4000");

    socket.on("connect", () => {
      console.log("Connected to server");
      socket.emit("join", roomId);
    });

    socket.on("message", (msg: Message) => {
      setMessages((prev) => [...prev, msg]);
    });

    setSocket(socket);

    return () => {
      socket.disconnect();
    };
  }, [roomId]);

  const sendMessage = (e: React.FormEvent) => {
    e.preventDefault();

    if (message.trim() && socket) {
      socket.emit("message", message);
      setMessage("");
    }
  };

  return (
    <div className="p-4 max-w-2xl mx-auto">
      <h1 className="text-2xl mb-4">Chat Room: {roomId}</h1>

      <div className="border rounded-lg p-4 h-[60vh] mb-4 overflow-y-auto">
        <ul className="p-4">
          {messages.map((msg, index) => (
            <li
              key={index}
              className="flex w-full justify-start gap-x-4 mb-4 align-top"
            >
              <div className="flex flex-col w-full">
                <div className="flex flex-row justify-between items-center">
                  <span
                    className={cn(
                      "text-sm font-semibold",
                      colorForName(msg.user)
                    )}
                  >
                    {msg.user}
                  </span>
                  <span className="text-muted-foreground inline-block ml-auto text-sm">
                    {new Date(msg.date).toLocaleString()}
                  </span>
                </div>
                <p className="text-ctp-text mt-1 text-lg">{msg.text}</p>
              </div>
            </li>
          ))}
        </ul>
      </div>

      <form onSubmit={sendMessage} className="flex gap-2">
        <Input
          type="text"
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          className="flex-1 p-2 border rounded"
          placeholder="Type a message..."
        />
        <Button
          type="submit"
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Send
        </Button>
      </form>
    </div>
  );
}
