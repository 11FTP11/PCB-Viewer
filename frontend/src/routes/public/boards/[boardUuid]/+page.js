// @ts-nocheck
import { error as svelteError, redirect } from '@sveltejs/kit';
import { serverProvidePublicBoard } from '$lib';
export async function load({ params }) {
    var boardUuid = params.boardUuid;

    return serverProvidePublicBoard(boardUuid)
        .then(
            (data) => {
                return { 
                    serverData: data,
                    guest: true
                }; 
            }
        )
        .catch(
            (err) => { 
                if (err && err["code"] == "info_not_found")
                    throw svelteError(404, "Not found")
                else {
                    throw svelteError(500, "Internal server error");
                }
            }
        );
}
