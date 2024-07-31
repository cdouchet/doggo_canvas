import { Room } from "../../../models/bindings/Room";

const getRoomState = async (room_id: string): Promise<Room> => {
  const fetchResult = await fetch("http://localhost:4576/rooms/" + room_id, {
    cache: "no-cache",
  });
  return await fetchResult.json();
};

export { getRoomState };
