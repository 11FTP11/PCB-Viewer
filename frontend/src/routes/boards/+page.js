// @ts-nocheck
import { error as svelteError, redirect } from '@sveltejs/kit';
import { serverBoardsMetadata } from '$lib';

/** @type {import('./$types').PageLoad} */
export async function load({ url }) {
    var page = parseInt(url.searchParams.get("page") || "1");
    var perPage = parseInt(url.searchParams.get("per_page") || "8");
    var rawCookie = `; ${document.cookie}`.split(`; userInfo=`).pop().split(';').shift() || "";
    var fullName = rawCookie ? decodeURIComponent(rawCookie) : "Пользователь";

    return serverBoardsMetadata(page, perPage)
        .then(
            (data) => {
                return { 
                    userInfo: fullName, 
                    serverData: data,
                    perPage: perPage
                }; 
            }
        )
        .catch(
            (err) => { 
                if (err && (err["code"] == "jwt_error" || err["code"] === "unauthorized")) {
                    if (err["code"] == "jwt_error")
                        document.cookie = "userInfo=; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT;"; 
                    
                    throw redirect(303, "/auth");
                }

                else {
                    throw svelteError(500, "Internal server error");
                }
            }
        );
}
