import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function generateRoomId() {
  // Generate a room ID in the format xxx-xxx with alphanumeric characters
  const chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
  return Array(6)
    .fill(0)
    .map(() => chars.charAt(Math.floor(Math.random() * chars.length)))
    .join("");
}

export function colorForName(name: string) {
  const colors = [
    "text-green-500",
    "text-pink-500",
    "text-red-500",
    "text-blue-500",
    "text-teal-500",
  ];

  name = name.toLowerCase();

  let sum = 0;
  for (let i = 0; i < name.length; i++) {
    sum += name.charCodeAt(i);
  }
  const index = sum % colors.length;

  return colors[index];
}
