/* @refresh reload */
import { render } from 'solid-js/web';

import './index.css';
import App from './App';
import { queryClient, rspc, rspcClient } from './rspc';

render(() => <rspc.Provider client={rspcClient} queryClient={queryClient}>
  <App />
</rspc.Provider>, document.getElementById('root') as HTMLElement);
