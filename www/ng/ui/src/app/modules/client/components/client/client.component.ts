import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { ApiResponse } from 'src/app/classes/api-response';
import { MessageType } from 'src/app/classes/message-type';
import { ClientService } from 'src/app/services/client.service';
import { MessageService } from 'src/app/services/message.service';
import { TitleService } from 'src/app/services/title.service';
import { UserService } from 'src/app/services/user.service';

@Component({
  selector: 'app-client',
  templateUrl: './client.component.html',
  styleUrls: ['./client.component.css']
})
export class ClientComponent implements OnInit {

  submitting = false;

  clients: Array<{
    id: string,
    name: string
  }> = new Array();

  constructor(
    private title: TitleService,
    private client_service: ClientService,
    private user_service: UserService,
    private msg_service: MessageService,
    private router: Router
  ) { }

  ngOnInit(): void {
    this.title.set_title("Join Client");
    this.fetch_clients();
  }

  fetch_clients() {
    this.client_service.clients(
      "%",
      true,
      10,
      1
    ).subscribe(r => {
      console.log("ClientComponent::refresh_clients()", r);
      if (r.success) {
        this.clients = (r.data as { clients: Array<{
          id: string,
          name: string,
          description: string,
          address: string,
          country_id: number,
          url: string
        }> }).clients;
      }
    });
  }

  join(client_id: string) {
    console.debug("ClientComponent::join()", client_id);
    this.submitting = true;
    this.client_service.user_join(client_id).subscribe((r: ApiResponse) => {
      if (r.success) {
        let client = this.clients.find(c => c.id == client_id);

        this.msg_service.send(`User joined client ${client?.name}`, MessageType.info);
        this.user_service.update();
        this.router.navigate([""]);
      } else {
        console.error("ClientComponent::join()", r);
      }
    });
  }
}
