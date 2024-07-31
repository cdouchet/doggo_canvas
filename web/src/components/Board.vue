<script setup lang="ts">
import { ref, onMounted } from "vue";
import { getRoomState } from "../api/room_api.ts";
import { Room } from "../../../models/bindings/Room";
import { User } from "../../../models/bindings/User";
import { UserLine } from "../../../models/bindings/UserLine";
import { CanvasState } from "../../../models/bindings/CanvasState";
import { useRouter } from "vue-router";

const originalCanvasWidth = 1920;
const originalCanvasHeight = 1080;

const sharedcanvas = ref<HTMLCanvasElement | null>(null);
const canvaswrapper = ref<HTMLCanvasElement | null>(null);

let users: Array<User> = [];

onMounted(() => {
    const splittedPath = location.pathname.split("board/");
    const boardId = splittedPath[1];
    if (boardId.length != 36) {
        const router = useRouter();
        router.push({ path: "/" });
    }

    const socket = new WebSocket(
        "ws://localhost:4576/ws?room_id=" + boardId + "&username=" + "didi",
    );

    const scaleCoordinates = (x: number, y: number) => {
        const scaleX = originalCanvasWidth / canvas.width;
        const scaleY = originalCanvasHeight / canvas.height;
        return { x: x * scaleX, y: y * scaleY };
    };

    const unscaleCoordinates = (x: number, y: number) => {
        const scaleX = canvas.width / originalCanvasWidth;
        const scaleY = canvas.height / originalCanvasHeight;
        return { x: x * scaleX, y: y * scaleY };
    };

    const addUser = (user: User) => {
        users.push(user);
        const newCanvas = document.createElement("canvas");
        console.log("user is: ", user);
        newCanvas.id = user.uid;
        newCanvas.width = window.innerWidth;
        newCanvas.height = window.innerHeight;
        newCanvas.style.position = "absolute";
        newCanvas.style.left = "0";
        newCanvas.style.top = "0";
        // const newCanvasContext = newCanvas.getContext("2d");
        canvaswrapper.value?.appendChild(newCanvas);
    };

    const drawUserLine = (userLine: UserLine) => {
        let userCanvas = document.getElementById(
            userLine.user_id,
        ) as HTMLCanvasElement | null;
        if (userCanvas == null) {
            userCanvas = sharedcanvas.value!;
        }
        const canvasContext = userCanvas.getContext("2d")!;
        canvasContext.strokeStyle = "black";
        const line = userLine.line;
        const from = unscaleCoordinates(line.from.x, line.from.y);
        const to = unscaleCoordinates(line.to.x, line.to.y);
        canvasContext.beginPath();
        canvasContext.moveTo(from.x, from.y);
        canvasContext.lineTo(to.x, to.y);
        canvasContext.stroke();
    };

    const drawRoomState = (state: CanvasState) => {
        for (const userLine of state) {
            drawUserLine(userLine);
        }
    };

    const addConnectedUsers = (users: Array<User>) => {
        for (let i = 0; i < users.length; i++) {
            addUser(users[i]);
        }
    };

    const setupRoomState = (room: Room) => {
        addConnectedUsers(room.users);
        drawRoomState(room.state);
    };

    const parseWebsocketMessage = (message: any) => {
        if ("user_join" in message) {
            const user: User = message.user_join;
            addUser(user);
        }
        if ("user_left" in message) {
            const uid = message.user_left;
            users = users.filter((e) => e.uid != uid);
            /// Deleting canvas would remove previous drawings
            // const userCanvas = document.getElementById(uid) as Node;
            // canvaswrapper.value?.removeChild(userCanvas);
        }
        if ("draw_line" in message) {
            drawUserLine(message.draw_line);
        } else {
            console.log("no draw");
            console.log(message);
        }
    };

    socket.onopen = async () => {
        console.log("Socket is opened! Fetching room info...");

        await new Promise((resolve) => setTimeout(() => resolve(null), 200));

        const room = await getRoomState(boardId);

        console.log("State is: ", room);

        setupRoomState(room);
    };

    socket.onmessage = (event) => {
        const data = JSON.parse(event.data);
        parseWebsocketMessage(data);
        // if ("draw_line" in data) {
        //     const from = data.draw_line.line.from;
        //     const to = data.draw_line.line.to;
        //     ctx.beginPath();
        //     ctx.moveTo(from.x, from.y);
        //     ctx.lineTo(to.x, to.y);
        //     ctx.stroke();
        // } else {
        //     console.log("no draw");
        //     console.log(data);
        // }
    };

    canvaswrapper.value!.style.position = "relative";
    canvaswrapper.value!.style.width = window.innerWidth.toString();
    canvaswrapper.value!.style.height = window.innerHeight.toString();

    const canvas = sharedcanvas.value!;
    const ctx = canvas.getContext("2d")!;
    ctx.strokeStyle = "black";

    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    // canvas.style.left = "0";
    // canvas.style.top = "0";
    canvas.style.position = "absolute";
    canvas.style.zIndex = "1000";

    let drawing = false;
    let lastPosition = { x: 0, y: 0 };

    canvas.addEventListener("mousedown", (e) => {
        drawing = true;
        lastPosition = { x: e.clientX, y: e.clientY };
    });

    canvas.addEventListener("mousemove", (e) => {
        if (!drawing) return;

        ctx.beginPath();
        ctx.moveTo(lastPosition.x, lastPosition.y);
        ctx.lineTo(e.clientX, e.clientY);
        ctx.stroke();

        socket.send(
            JSON.stringify({
                draw_line: {
                    from: scaleCoordinates(lastPosition.x, lastPosition.y),
                    to: scaleCoordinates(e.clientX, e.clientY),
                    thickness: "thin",
                    color: "black",
                },
            }),
        );
        lastPosition = { x: e.clientX, y: e.clientY };
    });

    canvas.addEventListener("mouseup", () => {
        drawing = false;
    });

    window.addEventListener("resize", () => {
        const foreignCanvases = users.map((e) =>
            document.getElementById(e.uid),
        ) as Array<HTMLCanvasElement>;
        const allCanvases = [canvas, ...foreignCanvases];
        for (let i = 0; i < allCanvases.length; i++) {
            allCanvases[i].width = window.innerWidth;
            allCanvases[i].height = window.innerHeight;
        }
    });
});
</script>

<template>
    <div ref="canvaswrapper">
        <canvas ref="sharedcanvas"></canvas>
    </div>
</template>

<style scoped></style>
