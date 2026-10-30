<script>
// @ts-nocheck

    import src from "$lib/assets/logo.svg";
    import { serverLogin } from "$lib/index.js"
    import { nonpassive } from "svelte/legacy";
    import { goto } from "$app/navigation";

    var handleLogin = (
        (event) => {
            event.preventDefault();
            signButtonDisabled = true;
            regFieldErrors.clearErrors();
            loginError = null;

            const data = new FormData(event.currentTarget, event.submitter);
            const email = data.get('email');
            const password = data.get('password');
            
            return serverLogin(email, password)
                .then(result => {
                    signButtonDisabled = false;
                    goto("/boards");
                })
                .catch(error => {
                    switch (error["code"]) {
                        case "invalid_credentials":
                            loginError = "Неверный адрес эл. почты или пароль";
                            break;
                        default:
                            loginError = "Проблемы с сервером, повторите запрос позже"
                            break;
                    }
                    signButtonDisabled = false;
                });
        }
    );

    var regFieldErrors = $state({
        firstName: null,
        lastName: null,
        email: null,
        password: null,
        regError: null
    });

    regFieldErrors.clearErrors = (
        function () {
           this.firstName = null; 
           this.lastName = null; 
           this.email = null; 
           this.password = null; 
           this.regError = null
        }
    );

    var loginError = $state(null);
    var signButtonDisabled = $state(false);
    let {data, form} = $props();
    var isLogin = $state(true);
</script>

<div id="blur-bg"></div>
<div id="form-container">
    <img {src} alt="logo: PCB viewer"/>
    
    <div class="buttons-container">
        <button 
            onclick={() => isLogin = true} 
            disabled={isLogin}
            class={isLogin ? 'active' : ''}
        >Вход в систему</button>
    </div>
    
    {#if isLogin}
        <form method="POST" onsubmit={handleLogin}>
            <label>
                <span>Эл. почта</span>
                <input name="email" maxlength="255" type="email"/>
            </label>
            <label>
                <span>Пароль</span>
                <input name="password" maxlength="128" type="password"/>
            </label>
            {#if loginError}
                <div class="error-message">{loginError}</div>
            {/if}
            <button type="submit" class="submit-form-button" disabled={signButtonDisabled}>Войти</button>
        </form>
    {/if}
</div>
<style> 
    #blur-bg {
        position: fixed;
        width: 100%;
        height: 100%;
        background-image: url("/patterns/lines_pcb.png");
        background-size: 180px 180px;
        background-repeat: repeat;
        filter: blur(1px);
        z-index: -2;
    }
    
    img {
        width: 90%;
        margin-bottom: 1.5rem; 
    }

    #form-container {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 30%;
        min-width: 320px;
        max-width: 35rem;
        border-radius: 1.5rem;
        background: rgba(255, 255, 255, 0.95);
        border: 1px solid rgba(0, 0, 0, 0.1);
        display: flex; 
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 2rem;
        box-shadow: 0 20px 35px -8px rgba(0, 0, 0, 0.2);
        backdrop-filter: blur(4px);
    }

    .buttons-container {
        display: flex;
        gap: 1rem;
        width: 100%;
    }

    .buttons-container button {
        flex: 1;
        padding: 0.75rem;
        border: 1px solid #d1d5db;
        background: white;
        border-radius: 0.75rem 0.75rem 0 0;
        cursor: pointer;
        font-size: 1rem;
        transition: background-color 0.3s, border-color 0.3s, color 0.3s;
    }

    .buttons-container button.active {
        background: #fb9c7d;
        border: none;
        color: white;
        cursor: default;
    }
    
    .buttons-container button:not(.active):hover {
        background-color: rgba(255, 187, 153, 0.508);
        border: 1px solid rgba(255, 187, 153, 0.508);
    }

    form {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        width: 100%;
        border: 2px solid #fb9c7d;
        border-radius: 0 0 1rem 1rem;
        background-color: whitesmoke;
        padding: 1rem;
        box-sizing: border-box;
    }

    label {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        font-size: 0.875rem;
        color: #374151;
        width: 100%;
        box-sizing: border-box;
    }

    input {
        width: 100%;
        box-sizing: border-box;
        padding: 0.5rem 0.75rem;
        border: 1px solid #d1d5db;
        border-radius: 0.5rem;
        font-size: 1rem;
        transition: all 0.2s;
        background: white;
    }

    input:focus {
        outline: none;
        border-color: #ffffff;
    }

    input.error {
        border-color: #dc2626;
        /* background: #fef2f2; */
    }

    .error-message {
        color: #dc2626;
        font-size: 0.75rem;
        margin-top: 0.25rem;
        min-height: 1.1rem;
    }

    .submit-form-button {
        margin-top: 0.5rem;
        padding: 0.75rem;
        background-color: #83D03F;
        color: white;
        border: none;
        border-radius: 0.5rem;
        font-size: 1rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.2s;
    }


    .submit-form-button:disabled {
        background-color: #d1d5db;
        cursor: not-allowed;
        transform: translateY(0);
    }

    .submit-form-button:not(.submit-form-button:disabled):hover {
        background-color: #6cb02e;
        transform: translateY(-1px);
    }

    .submit-form-button:active {
        transform: translateY(0);
    }



    .two-columns {
        display: flex;
        flex-direction: row;
        gap: 0.5rem;
        width: 100%;
        box-sizing: border-box;
    }

    .two-columns label {
        flex: 1;
        min-width: 0;
    }

    @media (max-width: 600px) {
        #form-container {
            width: 90%;
            padding: 1rem;
        }
        
        .two-columns {
            flex-direction: column;
            gap: 0.5rem;
        }
    }
</style>
