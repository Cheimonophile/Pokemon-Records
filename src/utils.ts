import { startOfToday, formatISO } from 'date-fns';


export const todayStr = () => formatISO(startOfToday(), { representation: 'date' });