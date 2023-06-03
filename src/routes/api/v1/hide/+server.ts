import { error } from "@sveltejs/kit";
import { hideMessage } from "$lib/hidenly";

export async function POST({ request }) {
    const { shown, hidden } = await request.json();

    if (!shown) throw error(400, "shown is required");
    if (!hidden) throw error(400, "hidden is required");

    return new Response(hideMessage(shown, hidden));
}