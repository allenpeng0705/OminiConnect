import { Outlet } from '@tanstack/react-router';
import { useEffect, useRef } from 'react';
import { useClickAway, useKeyPressEvent } from 'react-use';

import { triggerClose } from '@/lib/events';
import { useGlobal } from '@/lib/store';
import NangoLogoSVG from '@/svg/logo.svg?react';

export const Layout: React.FC = () => {
    const ref = useRef<HTMLDivElement>(null);

    const {
        isEmbedded,
        showWatermark,
        isAuthLink,
        setSessionToken,
        setApiURL,
        setWebsocketsPath,
        setDetectClosedAuthWindow,
        setIsEmbedded,
        setIsPreview,
        setAuthLink
    } = useGlobal();

    /** `/go` and other routes skip `Home`; still need query hydration so `@nangohq/frontend` uses the right host + WS. */
    useEffect(() => {
        const search = new URLSearchParams(window.location.search);
        const inUrl = search.get('session_token');
        if (inUrl) {
            setSessionToken(inUrl);
        }
        const apiU = search.get('apiURL');
        if (apiU) {
            setApiURL(apiU);
        }
        const wsPath = search.get('websocketsPath');
        if (wsPath) {
            setWebsocketsPath(wsPath);
        }
        const dc = search.get('detectClosedAuthWindow');
        if (dc) {
            setDetectClosedAuthWindow(dc === 'true');
        }
        const emb = search.get('embedded');
        if (emb) {
            setIsEmbedded(emb === 'true');
        }
        if (search.get('preview') === 'true') {
            setIsPreview(true);
        }
        setAuthLink(window.self === window.top);
    }, [
        setSessionToken,
        setApiURL,
        setWebsocketsPath,
        setDetectClosedAuthWindow,
        setIsEmbedded,
        setIsPreview,
        setAuthLink
    ]);
    const isDarkTheme = document.documentElement.classList.contains('dark');

    useClickAway(ref, () => {
        triggerClose('click:outside');
    });

    useKeyPressEvent('Escape', () => {
        triggerClose('click:outside');
    });

    if (isEmbedded) {
        return (
            <div ref={ref} className="h-screen w-screen flex flex-col max-w-[500px] max-h-[700px] rounded-md bg-elevated p-px overflow-hidden">
                <div className="flex-1 w-full bg-surface text-text-primary rounded-md -only:rounded-b-none overflow-y-auto">
                    <div className="min-h-full p-10 flex flex-col">
                        <Outlet />
                    </div>
                </div>
                {showWatermark && (
                    <div className="p-5 w-full text-center">
                        <a
                            className="shrink-0 text-xs text-text-tertiary"
                            href="https://www.nango.dev?utm_source=connectui"
                            rel="noopener noreferrer"
                            target="_blank"
                        >
                            Secured by
                            <NangoLogoSVG className="h-4 w-auto inline-block ml-2" />
                        </a>
                    </div>
                )}
            </div>
        );
    }

    return (
        <div
            className={`absolute h-screen w-screen overflow-hidden flex flex-col justify-center items-center sm:p-14 ${isAuthLink ? (isDarkTheme ? 'bg-black' : 'bg-gray-100') : 'bg-subtle/80'}`}
        >
            <div ref={ref} className="flex flex-col w-full h-full sm:w-[500px] sm:h-[700px] sm:rounded-md bg-elevated p-px overflow-hidden">
                <div className="flex-1 w-full bg-surface text-text-primary sm:rounded-md -only:rounded-b-none overflow-y-auto">
                    <div className="min-h-full p-5 sm:p-10 flex flex-col">
                        <Outlet />
                    </div>
                </div>
                {showWatermark && (
                    <div className="p-5 w-full text-center">
                        <a
                            className="shrink-0 text-xs text-text-tertiary"
                            href="https://www.nango.dev?utm_source=connectui"
                            rel="noopener noreferrer"
                            target="_blank"
                        >
                            Secured by
                            <NangoLogoSVG className="h-4 w-auto inline-block ml-2" />
                        </a>
                    </div>
                )}
            </div>
        </div>
    );
};
