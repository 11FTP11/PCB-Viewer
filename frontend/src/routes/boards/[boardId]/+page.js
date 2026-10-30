// @ts-nocheck
import { error as svelteError, redirect } from '@sveltejs/kit';
import { serverProvideBoard } from '$lib';

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
    var boardId = params.boardId;

    var rawCookie = `; ${document.cookie}`.split(`; userInfo=`).pop().split(';').shift() || "";
    var fullName = rawCookie ? decodeURIComponent(rawCookie) : "Пользователь";

    return serverProvideBoard(boardId) 
        .then(
            (data) => {
                return { 
                    userInfo: fullName, 
                    serverData: data,
                    guest: false
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

                if (err && err["code"] == "info_not_found")
                    throw svelteError(404, "Not found")

                else {
                    throw svelteError(500, "Internal server error");
                }
            }
        );
}
