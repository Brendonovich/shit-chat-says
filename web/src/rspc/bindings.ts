// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.

export type Operations = {
    queries: 
        { key: ["allMessages"], result: Array<Message> },
    mutations: never,
    subscriptions: never
};

export interface Message { id: number, content: string, sender: string }
