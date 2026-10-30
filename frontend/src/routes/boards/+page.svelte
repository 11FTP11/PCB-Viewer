<script>
// @ts-nocheck

    import logo from "$lib/assets/logo.svg";
    import logout from "$lib/assets/logout.svg";
    import trash  from "$lib/assets/trash.svg";
    import copySvg from "$lib/assets/copy.svg";
    import { goto, invalidateAll } from '$app/navigation';
    import { fade } from "svelte/transition";
    
    import { serverLogout } from "$lib";
    import { serverDeleteBoard } from "$lib";
    import { serverUploadBoard } from "$lib";

    var { data, form } = $props();
    var openMenuId = $state(null);
    var userInfo = data.userInfo;

    var boards = $state(data.serverData.data.page_data || []);
    
    $effect(() => {
        boards = data.serverData.data.page_data || [];
    });

    var totalPages = $derived(data.serverData.data.total_pages || 1);
    var currentPage = $derived(data.serverData.data.page);

    var startPage = $derived(Math.max(1, currentPage - 2));
    var endPage = $derived(Math.min(totalPages, startPage + 4));
    var finalStart = $derived(Math.max(1, endPage - 4));
    var visiblePages = $derived(
        Array.from({ length: (endPage - finalStart) + 1 }, (_, i) => finalStart + i)
    );

    var modalButtonDisabled = $state(false);
    var modalWindowOpen = $state(false);
    var modalErrors = $state({
        title: null,
        file: null
    })

    modalErrors.clearFields = (
        () => {
            modalErrors.title = null;
            modalErrors.file = null;
        }
    );

    var jwtErrorCheck = (
        (error) => {
            if (error["code"] == "jwt_error") {
                fetch("?/deleteToken");
            }
        }
    );

    var handleDelete = (
        (event) => {
            event.preventDefault();

            const formData = new FormData(event.currentTarget, event.submitter);
            const boardId = formData.get('boardId');
            openMenuId = null; 
            
            
            serverDeleteBoard(boardId)
                .then(result => invalidateAll())
                .catch(error => {
                    jwtErrorCheck(error);
                });
        }
    );

    var copyValue = ((value) => {
        var url = (`http://localhost:5173/public/boards/${value}`);
        navigator.clipboard.writeText(url);
        openMenuId = null;
    });

    var handleUpload = (
        (event) => {
            event.preventDefault();
            modalErrors.clearFields();
            modalButtonDisabled = true;

            const formData = new FormData(event.currentTarget, event.submitter);
            const fileTitle = formData.get('title');
            const file = formData.get("file");
            
            serverUploadBoard(file, fileTitle)
                .then(
                    result => { 
                        modalWindowOpen = false;
                        modalButtonDisabled = false;
                        invalidateAll();
                    }
                )
                .catch(
                    (error) => {
                        jwtErrorCheck(error);
                        
                        switch (error.code) {
                            case "pcad_parser_error":
                                modalErrors.file = "Некорректный файл"
                                break;
                            case "bad_file_title":
                                modalErrors.title = "Некорректное название файла"
                                break;
                            default:
                                modalErrors.file = "Ошибка, повторите запрос позже"
                                break;
                        }

                        modalButtonDisabled = false;
                    }
                );
        }
    );

    var handleView = (
        (event) => {
            event.preventDefault();
            modalErrors.clearFields();
            modalButtonDisabled = true;

            const formData = new FormData(event.currentTarget, event.submitter);
            const boardId = formData.get('boardId');
            goto(`/boards/${boardId}`);
        }
    );

    var handleLogout = (
        (event) => {
            if (event) event.preventDefault();
            serverLogout().then(
                () => goto("/auth")
            )
            .catch(
                (error) => {
                    console.error("Ошибка при выходе из аккаунта:", error);
                    alert("Не удалось выйти из системы");
                }
            );
        }
    );

    
</script>

<div id="bg"></div>

<div id="page-body">
    <header>
        <img id="logo" src={ logo } alt="logo: PCB viewer"/>
        <div id="user-container">
            <span>{ userInfo }</span>
            <form method="POST" onsubmit={handleLogout}>
                <button id="logout-button" type="submit">
                    <img id="logout" src={ logout } alt="logout"/>
                </button>
            </form>
        </div>
    </header>

    <main id="boards-menu">
        <div class="menu-header">
            <h2 id="page-name">МОИ ПЛАТЫ</h2>
                <button id="add-board-button"  onclick={() => modalWindowOpen = true}>
                    <span class="plus">+</span> Добавить схему
                </button>
        </div>

        <div class="boards-grid">
            {#each boards as item (item.id)}
                <div class="board-card">
                    <div class="card-info">
                        <span class="board-date">Добавлено: { new Date(item.added_at).toLocaleDateString("ru-Ru") }</span>
                        <span class="board-date">Обновлено: { new Date(item.updated_at).toLocaleString("ru-RU") }</span>
                        <span class="board-title">{ item.title }</span>
                    </div>
                    <div class="card-button-container">
                        <form onsubmit={handleView}>
                            <input name="boardId" value="{item.id}" type="hidden">
                            <button type="submit" class="card-open-button">Просмотр</button>
                        </form>
                        <div class="card-other-container">
                            <button class="card-other-button" onclick={ () => openMenuId = openMenuId === item.id ? null : item.id }>
                                <span>&hellip;</span>
                            </button>
                            {#if openMenuId == item.id}
                                <div class="card-dropdown-menu">
                                    <form onsubmit={handleDelete}>
                                        <input name="boardId" type="hidden" value="{item.id}"/>
                                        <button type="submit" class="delete-button">
                                            <span>Удалить</span>
                                            <img src={ trash } alt="trash"/>
                                        </button>
                                    </form>
                                    <button onclick={() => copyValue(item.public_token)} class="copy-button">
                                        <span>Копировать ссылку</span>
                                        <img src={ copySvg } alt="trash"/>
                                    </button>
                                </div>
                            {/if}
                        </div>
                    </div>
                </div>
            {:else}
                <div class="empty-state">
                    <p>У вас пока нет загруженных схем</p>
                </div>
            {/each}
        </div>
        <div id="pagination">
            <a 
                href="/boards?page={currentPage - 1}&per_page={data.perPage}"
                class="page-link nav-button {currentPage <= 1 ? 'disabled' : ''}"
                data-sveltekit-reload
            >&leftarrow;</a>

            {#each visiblePages as pageNum (pageNum)} 
                <a 
                    class="page-link {currentPage === pageNum ? 'active' : ''}"
                    href="/boards?page={pageNum}&per_page={data.perPage}" 
                    data-sveltekit-reload
                >
                    { pageNum }
                </a>
            {/each}

            <a 
                href="/boards?page={currentPage + 1}&per_page={data.perPage}"
                class="page-link nav-button {currentPage >= totalPages ? 'disabled' : ''}"
                data-sveltekit-reload
            >&rightarrow;</a>
        </div>

    </main>
</div>
{#if modalWindowOpen}
    <div id="modal-overlay" onclick={() => modalWindowOpen = false}
        transition:fade={{ duration: 200 }}
    >
        <div id="modal-window" onclick={(e) => e.stopPropagation()}>
            <div id="modal-header">
                <h3>Добавить схему</h3>
                <button class="close-btn" onclick={() => modalWindowOpen = false}>&times;</button>
            </div>
            <div id="modal-body">
                <form onsubmit={handleUpload}>
                    <div class="field-group">
                        <label>Название схемы (Опционально)
                            <input class="{modalErrors.title ? "error" : ""}" type="text" name="title" />
                        </label>
                        {#if modalErrors.title}
                            <div class="error-message">{ modalErrors.title }</div>
                        {/if}
                    </div>

                    <div class="field-group">
                        <label>Файл (ACCEL ASCII 2006 .pcb)
                            <input class="{modalErrors.file ? "error" : ""}" type="file" name="file" accept=".pcb"/>
                        </label>
                        {#if modalErrors.file}
                            <div class="error-message">{ modalErrors.file }</div>
                        {/if}
                    </div>

                    <button disabled={modalButtonDisabled} type="submit" id="modal-submit-button">Загрузить</button>
                </form>
            </div> 
        </div>
    </div>
{/if}

<style>

    header {
        background-color: white;
        margin-bottom: 0.5rem;
        border-radius: 1.5rem;
        display: flex;
        align-items: center;
        box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
        border: 1px solid rgba(0, 0, 0, 0.1);
        padding-left: 2rem;
        padding-right: 2rem;
        justify-content: space-between;
    }

    #modal-body {
        padding: 1.5rem;
        display: flex;
        flex-direction: column;
    }

    #modal-overlay {
        position: fixed; 
        
        top: 0;
        left: 0;
        width: 100vw; 
        height: 100vh;
        
        z-index: 9999; 

        background: rgba(0, 0, 0, 0.5); 
        backdrop-filter: blur(4px);    
        display: flex;           
        justify-content: center; 
        align-items: center;     
    }


    #modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        background: #fb9c7d; 
        border-radius: 0.5rem 0.5rem 0 0;
        align-items: center;
        padding-left: 1rem;
        padding-right: 1rem;
        color: white;
        word-spacing: 2px;
    }


    .close-btn {
        background: none;
        border: none;
        font-size: 1.5rem;
        cursor: pointer;
        color: #9ca3af;
    }

    .error-message {
        color: #dc2626;
        font-size: 0.75rem;
        margin-top: 0.25rem;
        min-height: 1.1rem;
    }

    form {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    input {
        padding: 0.5rem;
        border: 1px solid #d1d5db;
        border-radius: 0.4rem;
    }

    #modal-submit-button {
        background-color: #83D03F;
        color: white;
        border: none;
        padding: 0.75rem;
        border-radius: 0.5rem;
        font-weight: 600;
        cursor: pointer;
        transition: transform 0.2s, background-color 0.2s;
    }

    #modal-submit-button:disabled {
        background-color: #d1d5db;
        cursor: not-allowed;
        transform: translateY(0);
    }

    #modal-submit-button:not(#modal-submit-button:disabled):hover {
        background-color: #83D03F;
        transform: translateY(-2px);
    }

    #modal-window {
        background: white;
        border-radius: 1rem; 
        border: 2px solid #fb9c7d;
        box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
        width: 100%;
        max-width: 400px; 
        position: relative; 
        background: whitesmoke;

        label {
            display: flex;
            flex-direction: column;

            input {
                background: white;
            }

            input:focus {
                outline: none;
                border-color: #ffffff;
            }

            input.error {
                border-color: #dc2626;
            }
        }
    }


    #user-container {
        display: flex;
        align-items: center;
        flex-direction:row;
        gap: 0.5rem;
    }

    #logout-button {
        background: none;       
        border: none;           
        padding: 0;             
        margin: 0;
        cursor: pointer;
        
        display: flex;
        align-items: center;
        justify-content: center;

        width: 2rem; 
        height: 2rem;
        border-radius: 0.5rem;   
        transition: background-color 0.2s;
    }

    #logout-button:hover {
        background-color: #fb9c7d8b; 
    }

    #logout-button img {
        width: 50%;  
        height: 50%;
        object-fit: contain; 
    }

    #bg {
        position: fixed;
        width: 100%;
        height: 100%;
        z-index: -2;
        background-color: #d1d5db;
    }

    #page-body {
        margin: 0 auto;
        padding: 1.5rem;
        font-family: sans-serif;
        display: flex;
        flex-direction: column;
        min-height: 100vh; 
        box-sizing: border-box;
    }

    #logo {
        width: 20%;
        min-width: 10rem;
    }

    #boards-menu {
        background: rgba(255, 255, 255, 0.95);
        border: 1px solid rgba(0, 0, 0, 0.1);
        border-radius: 1.5rem;
        padding: 2rem;
        box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
        backdrop-filter: blur(4px);
        min-height: 70vh;
        display: flex;
        flex-direction: column;
    }

    .menu-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    #page-name {
        background: #fb9c7d; 
        border-radius: 0.5rem 0.5rem 0 0;
        margin: 0;
        color: white;
        padding: 0.75rem;
        letter-spacing: 1px;
    }

    #add-board-button {
        background-color: #83D03F; 
        color: white;
        border: none;
        padding: 0.75rem 1.5rem;
        border-radius: 0.75rem;
        font-weight: 600;
        cursor: pointer;
        transition: transform 0.2s, background-color 0.2s;
    }

    #pagination {
        display: flex;
        align-items: center;
        justify-content: center;
        margin-top: 0.5rem;
    }

    #add-board-button:hover {
        background-color: #83D03F;
        transform: translateY(-2px);
    }

    .boards-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(15rem, 1fr));
        grid-auto-rows: minmax(10rem, max-content);
        gap: 0.7rem;
        border: 2px solid #fb9c7d; 
        border-radius: 0 0.5rem 0.5rem 0.5rem;
        padding: 1rem;
        align-content: start;
        flex-grow: 1;
        background: whitesmoke;
    }

    .board-card {
        box-sizing: border-box;
        /* background: whitesmoke; */
        background: white;
        border: 1px solid #d1d5db;
        border-radius: 0.5rem;
        padding: 0.75rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
        transition: all 0.2s;
        height: 100%;
    }

    .board-card:hover {
        border-color: #fb9c7d;
        box-shadow: 0 5px 15px rgba(0,0,0,0.05);
    }

    .card-info {
        display: flex;
        flex-direction: column;
        flex-grow: 1;
    }

    .board-title {
        font-weight: 700;
        color: #1f2937;
        font-size: 1.1rem;
    }

    .board-date {
        font-size: 0.7rem;
        color: #6b7280;
        text-align: right;
    }

    .card-button-container {
        display: flex;    
        gap: 0.4rem;      
        align-items: stretch; 

        form {
            display: flex; 
            flex-direction: row;
            flex-grow: 1; 
        }
    }

    .card-other-container {
        display:flex;
        aspect-ratio: 1 / 1;
        position: relative;
    }

    .card-other-button {
        border-color: #fb9c7d;
        border:none;
        border-radius: 0.5rem;
        cursor: pointer;
        flex-grow: 1;
    }

    .card-open-button {
        background: #83D03F;
        color: white;
        border: none;
        padding: 0.5rem;
        width: 70%;
        border-radius: 0.5rem;
        cursor: pointer;
        font-weight: 600;
        transition: all 0.2s;
        flex-grow: 1;
    }

    .card-open-button:hover {
        background-color: #6cb02e;
        transform: translateY(-1px);
    }

    .card-open-button:active {
        transform: translateY(0);
    }



    .boards-grid:has(.empty-state) {
        display: flex;
        flex-direction: column;
        align-content: stretch; 
    }

    .empty-state {
        flex-grow: 1; 
        width: 100%;
        
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;

        text-align: center;
        padding: 4rem;
        color: #9ca3af;
        border: 2px dashed #d1d5db;
        border-radius: 1rem;
        box-sizing: border-box;
    }

    .card-dropdown-menu {
        position: absolute;
        top: 110%; 
        right: 0;  
        background: white;
        border: 1px solid #d1d5db;
        border-radius: 0.5rem;
        box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
        z-index: 10;
        min-width: 120px;
        display: flex;
        flex-direction: column;
        overflow: hidden;
        /* white-space: nowrap; */
    }

    .card-dropdown-menu button {
        padding: 0.5rem 1rem;
        border: none;
        background: none;
        text-align: left;
        cursor: pointer;
        font-size: 0.9rem;
        width: 100%;
    }

    .card-dropdown-menu button img {
        flex-shrink: 0;           
        width: 1rem;              
        height: 1rem;             
        object-fit: contain;      
        display: block;           
    }

    .card-dropdown-menu .copy-button,
    .card-dropdown-menu .delete-button {
        display: flex;
        gap: 0.2rem;
        justify-content: flex-start; 
        align-items: center;
    }

    .card-dropdown-menu button:hover {
        background-color: #f3f4f6;
    }

    #pagination {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 0.5rem;
        background-color: #fb9c7d;
        margin: 0;
        padding: 0.3rem;
        border-radius: 0 0 0.5rem 0.5rem;
        align-self: center;
    }

    .page-link, .nav-button {
        text-decoration: none;
        color: #374151;
        background: white;
        border: 1px solid #d1d5db;
        border-radius: 0.5rem;
        transition: all 0.2s;

        display: inline-flex;     
        align-items: center;      
        justify-content: center;   
        
        min-width: 2.5rem;        
        height: 2.5rem;           
        padding: 0;               
        
        font-size: 1rem;        
    }

    .page-link:hover:not(.active), .nav-btn:hover:not(.disabled) {
        background-color: #f3f4f6;
        border-color: #fb9c7d;
    }

    .page-link.active {
        opacity: 0.5;
        pointer-events: none; 
        background-color: #f9fafb;
    }

    .nav-button.disabled {
        opacity: 0.5;
        pointer-events: none; 
        background-color: #f9fafb;
    }
</style>
