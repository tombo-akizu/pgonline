"use client"

import React, { Fragment, useEffect } from 'react';
import { Unity, useUnityContext } from "react-unity-webgl";

import { SERVER_IP_ADDRESS } from '@/consts';

function App() {
  const { unityProvider, loadingProgression, isLoaded, sendMessage } = useUnityContext({
    loaderUrl: "unity/Build/build.loader.js",
    dataUrl: "unity/Build/build.data.unityweb",
    frameworkUrl: "unity/Build/build.framework.js.unityweb",
    codeUrl: "unity/Build/build.wasm.unityweb",
  });

  useEffect(() => {
    sendMessage("Connection", "Initialize", SERVER_IP_ADDRESS);
  }, [isLoaded]);

  return (
    <Fragment>
      {!isLoaded && (
        <p>Loading Application... {Math.round(loadingProgression * 100)}%</p>
      )}
      <Unity
        unityProvider={unityProvider}
        style={{
          width: "100%",
          visibility: isLoaded ? "visible" : "hidden"
        }}
      />
    </Fragment>
  );
}

export default App;