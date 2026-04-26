import { useEffect, useRef } from 'react';

import Nango from '@nangohq/frontend';

import { useGlobal } from './store';

export function useNango() {
    const sessionToken = useGlobal((state) => state.sessionToken);
    const setNango = useGlobal((state) => state.setNango);
    const nango = useGlobal((state) => state.nango);
    const apiURL = useGlobal((state) => state.apiURL);
    const websocketsPath = useGlobal((state) => state.websocketsPath);

    // Keep a ref to the latest nango instance so callbacks always use the current value.
    const nangoRef = useRef(nango);
    nangoRef.current = nango;

    // Create a singleton; must react to apiURL / websocketsPath (Home may set apiURL after sessionToken).
    useEffect(() => {
        if (!sessionToken) {
            return;
        }

        const opts: ConstructorParameters<typeof Nango>[0] = {
            connectSessionToken: sessionToken,
            host: apiURL
        };
        if (websocketsPath) {
            opts.websocketsPath = websocketsPath;
        }

        const newNango = new Nango(opts);
        setNango(newNango);
        nangoRef.current = newNango;
    }, [sessionToken, apiURL, websocketsPath, setNango]);

    return nangoRef.current;
}
