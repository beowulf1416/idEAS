import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';
import { MessageType } from '../classes/message-type';


@Injectable({
  providedIn: 'root'
})
export class MessageService {

  message = new BehaviorSubject<{ message: string, type: MessageType }>({ message: '', type: MessageType.info });

  constructor() { }

  get message$() {
    return this.message.asObservable();
  }

  send(msg: string, type: MessageType) {
    this.message.next({ 
      message: msg,
      type: type
    });
  }
}
