import { error } from "@sveltejs/kit";
import { unhideMessage } from "$lib/hidenly";

export async function POST({ request }) {
    const { encoded } = await request.json();

    if (!encoded) throw error(400, "encoded is required");

    return new Response(unhideMessage(message));
}