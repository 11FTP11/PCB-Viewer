// @ts-nocheck
// place files you want to import through the `$lib` alias in this folder.

export var serverLogin = (
    (email, password) => {
        return fetch("http://localhost:4000/api/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ email, password }),
            credentials: "include"
        }).then(result => {
            if (!result.ok) {
                return result.json()
                    .then(err => Promise.reject(err));
            }
            return result.json();
        });
    }
);

export var serverRegister = (
    (firstName, lastName, email, password) => {
        return fetch("http://localhost:4000/api/register", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(
                { 
                    "firstName": firstName, 
                    "lastName": lastName, 
                    "email": email, 
                    "password": password 
                }
            ),
            credentials: "include"
        }).then((result) => {
            if (!result.ok) {
                return result.json()
                    .then(err => Promise.reject(err));
            }
            return result.json();
        });
    }
);

export var serverLogout = (
        () => fetch("http://localhost:4000/api/logout", {
            method: "GET",
            credentials: "include"
        }).then((result) => {
            if (!result.ok) {
                return result.json()
                    .then(err => Promise.reject(err));
            }
            return result.json();
        })
    );

export var serverBoardsMetadata = (
    (page, perPage) => {
        return fetch(
            `http://localhost:4000/api/boards?per_page=${perPage}&page=${page}`,
            {
                method: "GET",
                credentials: "include"
            }
        ).then((result) => {
            if (!result.ok) {
                return result.json()
                    .then(err => Promise.reject(err));
            }
            return result.json();
        });
    }
);

export var serverDeleteBoard = (
    (boardId) => {
        return fetch(
            `http://localhost:4000/api/boards/${boardId}`,
            {
                method: "DELETE",
                credentials: "include"
            }
        ).then((result) => {
            if (!result.ok) {
                return result.json()
                    .then(err => Promise.reject(err));
            }
            return result.json();
        });
    }
);

export var serverUploadBoard = (
    (file, fileTitle) => {
        const myForm = new FormData();
        myForm.append("file", file);
        myForm.append("title", fileTitle);
        
        return fetch(
            `http://localhost:4000/api/boards`,
            {
                method: "POST",
                credentials: "include",
                body: myForm 
            }
        ).then((result) => {
            if (!result.ok) {
                return result.json()
                    .then(err => Promise.reject(err));
            }
            return result.json();
        });
    }
);

export var serverProvideBoard = (
    (boardId) => {
        return fetch(
            `http://localhost:4000/api/boards/${boardId}`,
            {
                method: "GET",
                credentials: "include"
            }
        ).then((result) => {
            if (!result.ok) {
                return result.json()
                    .then(err => Promise.reject(err));
            }
            return result.json();
        });
    }
);

export var serverProvidePublicBoard = (
    (boardUuid) => {
        return fetch(
            `http://localhost:4000/api/public/boards/${boardUuid}`,
            {
                method: "GET",
            }
        ).then((result) => {
            if (!result.ok) {
                return result.json()
                    .then(err => Promise.reject(err));
            }
            return result.json();
        });
    }
);

export var updateBoard = (
    (save, boardId) => {
        return fetch(
            `http://localhost:4000/api/boards/${boardId}`,
            {
                method: "PUT",
                headers: {
                    "Content-Type": "application/json"
                },
                credentials: "include",
                body: JSON.stringify(
                    {
                        ref_defs: save.refDefs,
                        common_side_order: save.commonSideOrder,
                        mirror_side_order: save.mirrorSideOrder,
                        rotation: save.rotation
                    }
                )
            }
        ).then((result) => {
            if (!result.ok) {
                return result.json()
                    .then(err => Promise.reject(err));
            }
            return result.json();
        });
    }
);
