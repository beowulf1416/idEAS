import { Component } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute, Router } from '@angular/router';
import { ApiResponse } from 'src/app/classes/api-response';
import { TitleService } from 'src/app/services/title.service';
import { ClientService } from '../../services/client.service';

@Component({
  selector: 'app-member-invite',
  templateUrl: './member-invite.component.html',
  styleUrls: ['./member-invite.component.css']
})
export class MemberInviteComponent {

  submitting = false;
  inviteForm = new FormGroup({
    client_id: new FormControl('', []),
    email: new FormControl('', [
      Validators.required,
      Validators.email
    ])
  });

  constructor(
    private title: TitleService,
    private client_service: ClientService,
    private route: ActivatedRoute,
    private router: Router
  ) {
    this.title.set_title("Invite User");

    const client_id = this.route.snapshot.paramMap.get('client_id') || '';
    if (client_id == '') {
      console.log("MemberInviteComponent::MemberInviteComponent(): client_id is empty");
      this.router.navigate(['/admin/clients']);
    } else {
      this.inviteForm.get('client_id')?.setValue(client_id);
    }
  }

  submit() {
    console.log("MemberInviteComponent::submit()");
    if (this.inviteForm.valid) {
      this.submitting = true;
      const client_id = this.inviteForm.get('client_id')?.value || '';
      this.client_service.invite_member(
        client_id,
        this.inviteForm.get('email')?.value || ''
      ).subscribe((r: ApiResponse) => {
        if (r.success) {
          this.router.navigate([`/admin/clients/client/members/${client_id}`]);
        } else {
          console.error("MemberInviteComponent::submit()", r);
        }
        this.submitting = false;
      });
    }
  }

  get client_id() {
    return this.inviteForm.get('client_id');
  }

  get email() {
    return this.inviteForm.get('email');
  }
}
