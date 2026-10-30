// @ts-nocheck
/** @type {import('./$types').PageLoad} */
export async function load() {
    const hasUserInfo = document.cookie.includes("userInfo=");

    if (hasUserInfo) {
        throw redirect(303, "/boards");
    }
}